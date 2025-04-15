1. Initialize the Client

```rs
   // Set up program clients for the three main programs: Autocrat, ConditionalVault, and AMM
   let autocrat_client = ctx.accounts.autocrat_program.to_account_info();
   let vault_client = ctx.accounts.vault_program.to_account_info();
   let amm_client = ctx.accounts.amm_program.to_account_info();
```

2. Generate a Proposal Address

```rs
   // Create a unique nonce for the proposal
   let nonce = Clock::get()?.slot as u64; // Or use a random number generator

   // Find the proposal PDA
   let (proposal_pubkey, proposal_bump) = Pubkey::find_program_address(
       &[
           b"proposal",
           authority.key().as_ref(),
           &nonce.to_le_bytes()
       ],
       autocrat_program_id
   );
```

3. Initialize the Question Account

```rs
   // Create a question with SHA-256 hash of "Will [proposal_address] pass?/FAIL/PASS"
   let question_data = format!("Will {} pass?/FAIL/PASS", proposal_pubkey);
   let question_hash = sha256(question_data.as_bytes());

   // Invoke conditional_vault.initialize_question CPI
   conditional_vault::cpi::initialize_question(
       CpiContext::new(
           vault_client,
           conditional_vault::cpi::accounts::InitializeQuestion {
               payer: authority.to_account_info(),
               question: question_account.to_account_info(),
               system_program: system_program.to_account_info(),
           }
       ),
       question_hash.to_vec(),
       proposal_pubkey,
       2, // Number of outcomes (PASS/FAIL)
   )?;
```

4. Initialize the Conditional Vault

```rs
   // Initialize base token vault (dao.token_mint)
   conditional_vault::cpi::initialize_vault(
       CpiContext::new(
           vault_client,
           conditional_vault::cpi::accounts::InitializeVault {
               payer: authority.to_account_info(),
               question: question_account.to_account_info(),
               vault: base_vault.to_account_info(),
               underlying_token_mint: dao_token_mint.to_account_info(),
               system_program: system_program.to_account_info(),
               token_program: token_program.to_account_info(),
               // Include other required accounts
           }
       ),
       dao_token_mint.key(),
       2, // Number of outcomes
       authority.key(),
   )?;

   // Initialize quote token vault (dao.usdc_mint) - Similar to above
```

5.Initialize the AMMs

```rs
   // Get PDAs for pass and fail markets
   let (pass_amm, _) = get_pass_amm_pda(proposal_pubkey, program_id);
   let (fail_amm, _) = get_fail_amm_pda(proposal_pubkey, program_id);

   // Initialize PASS market AMM
   amm::cpi::initialize_amm(
       CpiContext::new(
           amm_client,
           amm::cpi::accounts::InitializeAmm {
               user: authority.to_account_info(),
               amm: pass_amm_account.to_account_info(),
               base_mint: pass_base_mint.to_account_info(),
               quote_mint: pass_quote_mint.to_account_info(),
               // Include other required accounts
           }
       ),
       dao.twap_start_delay_slots,
       dao.twap_initial_observation,
       dao.twap_max_observation_change_per_update,
   )?;

   // Initialize FAIL market AMM - Similar to above
```

6. Split Tokens

```rs
   // Split base tokens
   conditional_vault::cpi::split_tokens(
       CpiContext::new(
           vault_client,
           conditional_vault::cpi::accounts::SplitTokens {
               authority: authority.to_account_info(),
               question: question_account.to_account_info(),
               vault: base_vault.to_account_info(),
               // Include other required accounts
           }
       ),
       base_tokens_to_lp,
       2, // Outcomes
       authority.key(),
   )?;

   // Split quote tokens - Similar to above
```

7. Add Liquidity to Both Markets

```rs
   // Add liquidity to PASS market
   amm::cpi::add_liquidity(
       CpiContext::new(
           amm_client,
           amm::cpi::accounts::AddLiquidity {
               user: authority.to_account_info(),
               amm: pass_amm_account.to_account_info(),
               // Include other required accounts
           }
       ),
       pass_amm.key(),
       pass_base_mint.key(),
       pass_quote_mint.key(),
       quote_tokens_to_lp,
       base_tokens_to_lp,
       0, // min_lp
       authority.key(),
   )?;

   // Add liquidity to FAIL market - Similar to above
```

8. Initialize the Proposal

```rs
   // Create the instruction to be executed if proposal passes
   let instruction = ProposalInstruction {
       program_id: instruction_program_id,
       accounts: instruction_accounts,
       data: instruction_data,
   };

   // Initialize the proposal
   autocrat::cpi::initialize_proposal(
       CpiContext::new_with_signer(
           autocrat_client,
           autocrat::cpi::accounts::InitializeProposal {
               proposal: proposal_account.to_account_info(),
               dao: dao_account.to_account_info(),
               question: question_account.to_account_info(),
               quote_vault: quote_vault.to_account_info(),
               base_vault: base_vault.to_account_info(),
               pass_amm: pass_amm_account.to_account_info(),
               fail_amm: fail_amm_account.to_account_info(),
               // Include other required accounts
           },
           &[proposal_signer] // If needed
       ),
       InitializeProposalParams {
           description_url: description_url.to_string(),
           instruction,
           pass_lp_tokens_to_lock,
           fail_lp_tokens_to_lock,
           nonce,
       },
   )?;
```
