import { useCallback } from "react";
import { utils } from "@coral-xyz/anchor";
import { useWallet } from "@solana/wallet-adapter-react";
import {
  AmmClient,
  AutocratClient,
  ConditionalVaultClient,
  getProposalAddr,
  InstructionUtils,
  ProposalInstruction,
} from "@metadaoproject/futarchy/v0.4";
import { useProvider } from "@/hooks/useProvider";
import {
  ComputeBudgetProgram,
  LAMPORTS_PER_SOL,
  PublicKey,
} from "@solana/web3.js";
import BN from "bn.js";
import { signAndSendTransactionWithToast } from "@/utils/txSendToast";
import { signAndSendTransactionsWithToast } from "@/utils/txSendToastMultiBundle";
import { useProposalStore } from "@/store/useProposalStore";
import { useAuthConnection } from "@/hooks/auth/useAuthConnection";
import { sha256 } from "@noble/hashes/sha256";

export const useV04ProposalTransactions = () => {
  const wallet = useWallet();
  const provider = useProvider();
  const store = useProposalStore();
  const { authenticate } = useAuthConnection();

  const getChainAmount = useCallback(
    (humanAmount: string, decimals: number) => {
      const [integerPart, fractionalPart = ""] = humanAmount.split(".");
      return new BN(integerPart + fractionalPart)
        .mul(new BN(10).pow(new BN(decimals)))
        .div(new BN(10).pow(new BN(fractionalPart.length)));
    },
    [],
  );

  const getProposalAddress = useCallback(
    (autocratClient: AutocratClient, nonce: BN): PublicKey | null => {
      if (!wallet.publicKey) return null;

      let proposal: PublicKey | null = store.onchainProposalDetails.proposalAcct
        ? new PublicKey(store.onchainProposalDetails.proposalAcct)
        : null;

      if (!proposal) {
        [proposal] = PublicKey.findProgramAddressSync(
          [
            utils.bytes.utf8.encode("proposal"),
            wallet.publicKey.toBuffer(),
            nonce.toArrayLike(Buffer, "le", 8),
          ],
          autocratClient.autocrat.programId,
        );
        store.setOnchainProposalDetails(proposal.toBase58(), nonce);
      }

      return proposal;
    },
    [
      wallet.publicKey,
      store.onchainProposalDetails,
      store.setOnchainProposalDetails,
    ],
  );

  const saveProposalDraft = useCallback(
    async (proposalAcct: PublicKey) => {
      if (!wallet.publicKey) return;

      const token = await authenticate();
      const endpoint = store.proposalDbId
        ? `/api/proposal/${store.proposalDbId}`
        : "/api/proposal";

      const response = await fetch(endpoint, {
        method: store.proposalDbId ? "PUT" : "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${token}`,
        },
        body: JSON.stringify({
          data: {
            title: store.title,
            description: store.description,
            categories: null,
            content: store.content,
            proposer_acct: wallet.publicKey.toBase58(),
            proposal_acct: proposalAcct.toBase58() ?? null,
            organization_id: store.selectedDao?.organizationId,
          },
        }),
      });

      const data = await response.json();
      console.log("Draft save response:", data);

      if (!data.success) {
        throw new Error(
          data.error?.message || "Failed to create proposal in db",
        );
      }

      if (!store.proposalDbId) {
        store.setProposalDbId(data.data.proposal_id);
      }

      return data;
    },
    [
      authenticate,
      wallet.publicKey,
      store.proposalDbId,
      store.title,
      store.content,
      store.description,
      store.selectedDao,
      store.setProposalDbId,
    ],
  );

  const createProposal = useCallback(
    async (daoAcct: string) => {
      if (!wallet.publicKey || !daoAcct) return;

      const autocratClient = AutocratClient.createClient({ provider });
      const vaultClient = ConditionalVaultClient.createClient({ provider });
      const ammClient = AmmClient.createClient({ provider });

      // TODO: These two MUST be saved together.. If nonce is changed then proposalAcct should be too...
      // TODO: This is a bit of a convoluted way to do this, we should likely just make sure they're stored together...
      const nonce =
        store.onchainProposalDetails.nonce ?? new BN(Math.random() * 2 ** 50);

      const [proposalAcct] = getProposalAddr(
        autocratClient.autocrat.programId,
        wallet.publicKey,
        nonce,
      );

      if (!proposalAcct) throw new Error("Unable to create proposal address");

      await saveProposalDraft(proposalAcct);

      // Liquidity amount calculations (original lines 206-239)
      let baseTokensToLP: BN | null = null;
      let quoteTokensToLP: BN | null = null;
      try {
        baseTokensToLP = getChainAmount(
          store.baseLiquidity.toString(),
          store.selectedDao?.baseDecimals ?? 0,
        );
        quoteTokensToLP = getChainAmount(
          store.quoteLiquidity.toString(),
          store.selectedDao?.quoteDecimals ?? 0,
        );
      } catch (e) {
        console.log("Error converting amounts:", e);
      }

      const dao = await autocratClient.getDao(new PublicKey(daoAcct));
      if (!baseTokensToLP || !quoteTokensToLP) {
        baseTokensToLP = dao.minBaseFutarchicLiquidity;
        quoteTokensToLP = dao.minQuoteFutarchicLiquidity;
      }

      // TODO: This is for memo instruction. If we're not generating and storing it in the store, we need to do it here..
      const ix = {
        programId: new PublicKey("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"),
        accounts: [],
        data: Buffer.from(store.instruction.memo || "", "utf8"),
      } as ProposalInstruction;

      // Transaction construction (original lines 267-555)
      const {
        baseVault,
        quoteVault,
        passAmm,
        failAmm,
        passBaseMint,
        passQuoteMint,
        failBaseMint,
        failQuoteMint,
        question,
      } = autocratClient.getProposalPdas(
        proposalAcct,
        dao.tokenMint,
        dao.usdcMint,
        new PublicKey(daoAcct),
      );

      const questionTx = await vaultClient
        .initializeQuestionIx(
          sha256(`Will ${proposalAcct} pass?/FAIL/PASS`),
          proposalAcct,
          2,
        )
        .accounts({
          payer: wallet.publicKey,
        })
        .preInstructions([
          ComputeBudgetProgram.setComputeUnitLimit({
            units: 30_000,
          }),
        ])
        .transaction();

      const initializeVaultsTx = await vaultClient
        .initializeVaultIx(question, dao.tokenMint, 2, wallet.publicKey)
        .accounts({
          payer: wallet.publicKey,
        })
        .preInstructions([
          ComputeBudgetProgram.setComputeUnitLimit({
            units: 420_000,
          }),
        ])
        .postInstructions(
          await InstructionUtils.getInstructions(
            vaultClient
              .initializeVaultIx(question, dao.usdcMint, 2, wallet.publicKey)
              .accounts({
                payer: wallet.publicKey,
              }),
            ammClient
              .initializeAmmIx(
                passBaseMint,
                passQuoteMint,
                dao.twapStartDelaySlots,
                dao.twapInitialObservation,
                dao.twapMaxObservationChangePerUpdate,
              )
              .accounts({ user: wallet.publicKey }),
            ammClient
              .initializeAmmIx(
                failBaseMint,
                failQuoteMint,
                dao.twapStartDelaySlots,
                dao.twapInitialObservation,
                dao.twapMaxObservationChangePerUpdate,
              )
              .accounts({ user: wallet.publicKey }),
          ),
        )
        .transaction();

      // Conditional tokens?

      const splitTokensTx = await vaultClient
        .splitTokensIx(
          question,
          baseVault,
          dao.tokenMint,
          baseTokensToLP,
          2,
          wallet.publicKey,
        )
        .accounts({
          authority: wallet.publicKey,
        })
        .preInstructions([
          ComputeBudgetProgram.setComputeUnitLimit({
            units: 220_000,
          }),
        ])
        .postInstructions(
          await InstructionUtils.getInstructions(
            vaultClient
              .splitTokensIx(
                question,
                quoteVault,
                dao.usdcMint,
                quoteTokensToLP,
                2,
                wallet.publicKey,
              )
              .accounts({
                authority: wallet.publicKey,
              }),
          ),
        )
        .transaction();

      const liquidityTx = await ammClient
        .addLiquidityIx(
          passAmm,
          passBaseMint,
          passQuoteMint,
          quoteTokensToLP,
          baseTokensToLP,
          new BN(0),
          wallet.publicKey,
        )
        .accounts({
          user: wallet.publicKey,
        })
        .preInstructions([
          ComputeBudgetProgram.setComputeUnitLimit({
            units: 170_000,
          }),
        ])
        .postInstructions(
          await InstructionUtils.getInstructions(
            ammClient.addLiquidityIx(
              failAmm,
              failBaseMint,
              failQuoteMint,
              quoteTokensToLP,
              baseTokensToLP,
              new BN(0),
              wallet.publicKey,
            ),
          ),
        )
        .transaction();

      const proposalTx = await autocratClient
        .initializeProposalIx(
          store.url,
          ix,
          new PublicKey(daoAcct),
          dao.tokenMint,
          dao.usdcMint,
          quoteTokensToLP,
          quoteTokensToLP,
          nonce,
          question,
          wallet.publicKey,
        )
        .preInstructions([
          ComputeBudgetProgram.setComputeUnitLimit({
            units: 130_000,
          }),
        ])
        .transaction();

      // Transaction signing and sending logic (original lines 557-625)
      if (process.env.NEXT_PUBLIC_NETWORK !== "mainnet") {
        try {
          // TODO: If user rejects THIS AND ONLY THIS transaction then we don't nuke the proposal_acct
          // TODO: If user bails don't keep going down this chain of transactions

          await signAndSendTransactionWithToast(
            provider.connection,
            wallet,
            questionTx,
          );

          await signAndSendTransactionWithToast(
            provider.connection,
            wallet,
            initializeVaultsTx,
          );

          await signAndSendTransactionWithToast(
            provider.connection,
            wallet,
            splitTokensTx,
          );

          await signAndSendTransactionWithToast(
            provider.connection,
            wallet,
            liquidityTx,
          );

          // TODO: If we bailed above (rejected the transaction) then we don't expect to get here...
          // Make sure this only happens if this actually lands...
          store.setOnchainProposalDetails(null, null);
          // Beyond this step we should regenerate the proposal_acct
          await signAndSendTransactionWithToast(
            provider.connection,
            wallet,
            proposalTx,
          );
          console.log(proposalAcct.toBase58());

          if (wallet.publicKey) {
            store.clearDraft(wallet.publicKey.toString());
          }
          // Add success state update
          useProposalStore.setState({
            showSuccessDialog: true,
            showShareDialog: false,
            onchainProposalDetails: {
              proposalAcct: proposalAcct.toBase58(),
              nonce: nonce,
            },
          });
        } catch (e) {
          console.log(e);
          // TODO: If it fails AT all then proposal_acct is nuked and we need to purge to regenerate
          // TODO: If this is user reject first transaction then we can reuse, but should not refresh the page
          if (
            e instanceof Error &&
            e.message.includes("User rejected the request")
          ) {
            // don't clear the proposal_acct
          } else {
            // clear the proposal_acct
            if (wallet.publicKey) {
              store.setOnchainProposalDetails(null, null);
            }
          }
          useProposalStore.setState({
            showSuccessDialog: false,
            showShareDialog: false,
            onchainProposalDetails: {
              proposalAcct: proposalAcct.toBase58(),
              nonce: nonce,
            },
          });
          return;
        }
      } else {
        try {
          // TODO: Test send bundle
          await signAndSendTransactionsWithToast(
            provider.connection,
            wallet,
            [
              questionTx,
              initializeVaultsTx,
              splitTokensTx,
              liquidityTx,
              proposalTx,
            ],
            store.bundleTip
              ? Math.ceil(store.bundleTip * LAMPORTS_PER_SOL)
              : 7_500,
          );

          // Add this after successful bundle submission
          store.setOnchainProposalDetails(proposalAcct.toBase58(), nonce);

          if (wallet.publicKey) {
            store.clearDraft(wallet.publicKey.toString());
          }
          // Add success state update
          useProposalStore.setState({
            showSuccessDialog: true,
            showShareDialog: false,
            onchainProposalDetails: {
              proposalAcct: proposalAcct.toBase58(),
              nonce: nonce,
            },
          });
        } catch (e) {
          console.error(e);
          useProposalStore.setState({
            showSuccessDialog: false,
            showShareDialog: false,
          });
        }
      }
    },
    [
      getProposalAddress,
      saveProposalDraft,
      getChainAmount,
      provider,
      wallet,
      store.baseLiquidity,
      store.quoteLiquidity,
      store.selectedDao,
      store.bundleTip,
      store.clearDraft,
      store.setOnchainProposalDetails,
    ],
  );

  return {
    createProposal,
    getChainAmount,
    saveProposalDraft,
  };
};
