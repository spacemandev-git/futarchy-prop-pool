export type PropPool = {
  "version": "0.1.0",
  "name": "prop_pool",
  "instructions": [
    {
      "name": "createStagedProposal",
      "accounts": [
        {
          "name": "proposer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "stagedProp",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "stagedWallet",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rewardTokenBaseAta",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rewardTokenQuoteAta",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "proposerBaseTokenAta",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "proposerQuoteTokenAta",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "baseMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "quoteMint",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "args",
          "type": {
            "defined": "CreateStagedPropArgs"
          }
        }
      ]
    },
    {
      "name": "fundStagedProposal",
      "accounts": [
        {
          "name": "lp",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "stagedProp",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "lpAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "lpTokenAta",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "stagedPropAta",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "args",
          "type": {
            "defined": "FundStagedPropArgs"
          }
        }
      ]
    },
    {
      "name": "withdrawFromStagedProposal",
      "accounts": [
        {
          "name": "lp",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "stagedProp",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "lpAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "lpTokenAta",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "stagedPropAta",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "args",
          "type": {
            "defined": "WithdrawFromStagedPropArgs"
          }
        }
      ]
    },
    {
      "name": "initQuestion",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "stagedProp",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "stagedWallet",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "stagedPropBaseTokenAta",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "stagedPropQuoteTokenAta",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "questionAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "conditionalVault",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "conditionalVaultEventAuthority",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "args",
          "type": {
            "defined": "IxInitializeQuestionArgs"
          }
        }
      ]
    },
    {
      "name": "initConditionalVault",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "stagedProp",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "stagedWallet",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "conditionalVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "conditionalVaultEventAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "questionAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "baseVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vaultUnderlyingTokenAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "daoTokenMint",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "initAmms",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "stagedProp",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "stagedWallet",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "ammProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "ammEventAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "amm",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "lpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "baseMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "quoteMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "vaultAtaBase",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vaultAtaQuote",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "dao",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "splitTokens",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "stagedProp",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "stagedWallet",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "conditionalVault",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "conditionalVaultEventAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "questionAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "vault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vaultUnderlyingTokenAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "userUnderlyingTokenAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "addLiquidity",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "stagedProp",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "stagedWallet",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "amm",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "ammEventAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "lpMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "userLpAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "userBaseAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "userQuoteAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "vaultAtaBase",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "vaultAtaQuote",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "initProposal",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "stagedProp",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "stagedWallet",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "autocrat",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "autocratEventAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "proposal",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "dao",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "question",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "quoteVault",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "baseVault",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "passAmm",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "failAmm",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "passLpMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "failLpMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "passLpUserAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "failLpUserAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "passLpVaultAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "failLpVaultAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "args",
          "type": {
            "defined": "IxInitProposalArgs"
          }
        }
      ]
    },
    {
      "name": "executeProposal",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "stagedProp",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "autocrat",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "autocratEventAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "proposal",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "dao",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "finalizeProposal",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "stagedProp",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "stagedWallet",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "autocrat",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "autocratEventAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "proposal",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "dao",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "question",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "passAmm",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "failAmm",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "passLpUserAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "failLpUserAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "passLpVaultAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "failLpVaultAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "vaultProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "vaultEventAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "treasury",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "dao",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "treasuryPdaBump",
            "type": "u8"
          },
          {
            "name": "treasury",
            "type": "publicKey"
          },
          {
            "name": "tokenMint",
            "type": "publicKey"
          },
          {
            "name": "usdcMint",
            "type": "publicKey"
          },
          {
            "name": "proposalCount",
            "type": "u32"
          },
          {
            "name": "passThresholdBps",
            "type": "u16"
          },
          {
            "name": "slotsPerProposal",
            "type": "u64"
          },
          {
            "name": "twapInitialObservation",
            "docs": [
              "For manipulation-resistance the TWAP is a time-weighted average observation,",
              "where observation tries to approximate price but can only move by",
              "`twap_max_observation_change_per_update` per update. Because it can only move",
              "a little bit per update, you need to check that it has a good initial observation.",
              "Otherwise, an attacker could create a very high initial observation in the pass",
              "market and a very low one in the fail market to force the proposal to pass.",
              "",
              "We recommend setting an initial observation around the spot price of the token,",
              "and max observation change per update around 2% the spot price of the token.",
              "For example, if the spot price of META is $400, we'd recommend setting an initial",
              "observation of 400 (converted into the AMM prices) and a max observation change per",
              "update of 8 (also converted into the AMM prices). Observations can be updated once",
              "a minute, so 2% allows the proposal market to reach double the spot price or 0",
              "in 50 minutes."
            ],
            "type": "u128"
          },
          {
            "name": "twapMaxObservationChangePerUpdate",
            "type": "u128"
          },
          {
            "name": "twapStartDelaySlots",
            "docs": [
              "Forces TWAP calculation to start after amm.created_at_slot + twap_start_delay_slots"
            ],
            "type": "u64"
          },
          {
            "name": "minQuoteFutarchicLiquidity",
            "docs": [
              "As an anti-spam measure and to help liquidity, you need to lock up some liquidity",
              "in both futarchic markets in order to create a proposal.",
              "",
              "For example, for META, we can use a `min_quote_futarchic_liquidity` of",
              "5000 * 1_000_000 (5000 USDC) and a `min_base_futarchic_liquidity` of",
              "10 * 1_000_000_000 (10 META)."
            ],
            "type": "u64"
          },
          {
            "name": "minBaseFutarchicLiquidity",
            "type": "u64"
          },
          {
            "name": "seqNum",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "propLp",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "lpOwner",
            "type": "publicKey"
          },
          {
            "name": "stagedProp",
            "type": "publicKey"
          },
          {
            "name": "baseTokenContributed",
            "type": "u64"
          },
          {
            "name": "quoteTokenContributed",
            "type": "u64"
          },
          {
            "name": "rewardsRedeemed",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "stagedProposal",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "stagedId",
            "type": "u64"
          },
          {
            "name": "daoAddress",
            "type": "publicKey"
          },
          {
            "name": "proposer",
            "type": "publicKey"
          },
          {
            "name": "descriptionUrl",
            "type": "string"
          },
          {
            "name": "instruction",
            "type": {
              "defined": "StagedProposalInstruction"
            }
          },
          {
            "name": "stage",
            "type": {
              "defined": "StagedProposalState"
            }
          },
          {
            "name": "rewardTokens",
            "type": {
              "defined": "RewardTokens"
            }
          },
          {
            "name": "daoTokenThresholds",
            "type": {
              "defined": "DAOTokenThresholds"
            }
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "CreateStagedPropArgs",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "stagedId",
            "type": "u64"
          },
          {
            "name": "daoAddress",
            "type": "publicKey"
          },
          {
            "name": "descriptionUrl",
            "type": "string"
          },
          {
            "name": "instruction",
            "type": {
              "defined": "StagedProposalInstruction"
            }
          },
          {
            "name": "rewardTokens",
            "type": {
              "defined": "RewardTokens"
            }
          },
          {
            "name": "daoTokenThresholds",
            "type": {
              "defined": "DAOTokenThresholds"
            }
          }
        ]
      }
    },
    {
      "name": "FundStagedPropArgs",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "amount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "WithdrawFromStagedPropArgs",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "amount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "IxInitProposalArgs",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "nonce",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "IxInitializeQuestionArgs",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "proposalPubkey",
            "type": "publicKey"
          }
        ]
      }
    },
    {
      "name": "DAOTokenThresholds",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "baseMint",
            "type": "publicKey"
          },
          {
            "name": "quoteMint",
            "type": "publicKey"
          },
          {
            "name": "baseThreshold",
            "type": "u64"
          },
          {
            "name": "baseDecimals",
            "type": "u8"
          },
          {
            "name": "quoteThreshold",
            "type": "u64"
          },
          {
            "name": "quoteDecimals",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "RewardTokens",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "rewardTokenMint",
            "type": "publicKey"
          },
          {
            "name": "rewardTokenAmountBase",
            "type": "u64"
          },
          {
            "name": "rewardTokenAmountQuote",
            "type": "u64"
          },
          {
            "name": "rewardTokenDecimals",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "StagedProposalAccount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "pubkey",
            "type": "publicKey"
          },
          {
            "name": "isSigner",
            "type": "bool"
          },
          {
            "name": "isWritable",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "StagedProposalInstruction",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "programId",
            "type": "publicKey"
          },
          {
            "name": "accounts",
            "type": {
              "vec": {
                "defined": "StagedProposalAccount"
              }
            }
          },
          {
            "name": "data",
            "type": "bytes"
          }
        ]
      }
    },
    {
      "name": "StagedProposalState",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Staged"
          },
          {
            "name": "Market"
          },
          {
            "name": "Passed"
          },
          {
            "name": "Executed"
          },
          {
            "name": "Failed"
          },
          {
            "name": "Finalized"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "InvalidTokenMint",
      "msg": "Invalid token mint"
    },
    {
      "code": 6001,
      "name": "StagedPropAlreadyMarket",
      "msg": "Staged proposal already in market"
    },
    {
      "code": 6002,
      "name": "StagedPropNotLiquid",
      "msg": "Staged prop not liquid enough"
    },
    {
      "code": 6003,
      "name": "StagedPropStillFunding",
      "msg": "Cannot withdraw rewards if prop is still funding"
    }
  ]
};

export const IDL: PropPool = {
  "version": "0.1.0",
  "name": "prop_pool",
  "instructions": [
    {
      "name": "createStagedProposal",
      "accounts": [
        {
          "name": "proposer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "stagedProp",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "stagedWallet",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rewardTokenBaseAta",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rewardTokenQuoteAta",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "proposerBaseTokenAta",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "proposerQuoteTokenAta",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "baseMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "quoteMint",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "args",
          "type": {
            "defined": "CreateStagedPropArgs"
          }
        }
      ]
    },
    {
      "name": "fundStagedProposal",
      "accounts": [
        {
          "name": "lp",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "stagedProp",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "lpAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "lpTokenAta",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "stagedPropAta",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "args",
          "type": {
            "defined": "FundStagedPropArgs"
          }
        }
      ]
    },
    {
      "name": "withdrawFromStagedProposal",
      "accounts": [
        {
          "name": "lp",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "stagedProp",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "lpAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "lpTokenAta",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "stagedPropAta",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "args",
          "type": {
            "defined": "WithdrawFromStagedPropArgs"
          }
        }
      ]
    },
    {
      "name": "initQuestion",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "stagedProp",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "stagedWallet",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "stagedPropBaseTokenAta",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "stagedPropQuoteTokenAta",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "questionAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "conditionalVault",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "conditionalVaultEventAuthority",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "args",
          "type": {
            "defined": "IxInitializeQuestionArgs"
          }
        }
      ]
    },
    {
      "name": "initConditionalVault",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "stagedProp",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "stagedWallet",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "conditionalVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "conditionalVaultEventAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "questionAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "baseVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vaultUnderlyingTokenAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "daoTokenMint",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "initAmms",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "stagedProp",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "stagedWallet",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "ammProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "ammEventAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "amm",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "lpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "baseMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "quoteMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "vaultAtaBase",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vaultAtaQuote",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "dao",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "splitTokens",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "stagedProp",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "stagedWallet",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "conditionalVault",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "conditionalVaultEventAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "questionAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "vault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vaultUnderlyingTokenAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "userUnderlyingTokenAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "addLiquidity",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "stagedProp",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "stagedWallet",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "amm",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "ammEventAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "lpMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "userLpAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "userBaseAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "userQuoteAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "vaultAtaBase",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "vaultAtaQuote",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "initProposal",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "stagedProp",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "stagedWallet",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "autocrat",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "autocratEventAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "proposal",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "dao",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "question",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "quoteVault",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "baseVault",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "passAmm",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "failAmm",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "passLpMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "failLpMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "passLpUserAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "failLpUserAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "passLpVaultAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "failLpVaultAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "args",
          "type": {
            "defined": "IxInitProposalArgs"
          }
        }
      ]
    },
    {
      "name": "executeProposal",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "stagedProp",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "autocrat",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "autocratEventAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "proposal",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "dao",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "finalizeProposal",
      "accounts": [
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "stagedProp",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "stagedWallet",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "autocrat",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "autocratEventAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "proposal",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "dao",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "question",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "passAmm",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "failAmm",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "passLpUserAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "failLpUserAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "passLpVaultAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "failLpVaultAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "vaultProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "vaultEventAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "treasury",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "dao",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "treasuryPdaBump",
            "type": "u8"
          },
          {
            "name": "treasury",
            "type": "publicKey"
          },
          {
            "name": "tokenMint",
            "type": "publicKey"
          },
          {
            "name": "usdcMint",
            "type": "publicKey"
          },
          {
            "name": "proposalCount",
            "type": "u32"
          },
          {
            "name": "passThresholdBps",
            "type": "u16"
          },
          {
            "name": "slotsPerProposal",
            "type": "u64"
          },
          {
            "name": "twapInitialObservation",
            "docs": [
              "For manipulation-resistance the TWAP is a time-weighted average observation,",
              "where observation tries to approximate price but can only move by",
              "`twap_max_observation_change_per_update` per update. Because it can only move",
              "a little bit per update, you need to check that it has a good initial observation.",
              "Otherwise, an attacker could create a very high initial observation in the pass",
              "market and a very low one in the fail market to force the proposal to pass.",
              "",
              "We recommend setting an initial observation around the spot price of the token,",
              "and max observation change per update around 2% the spot price of the token.",
              "For example, if the spot price of META is $400, we'd recommend setting an initial",
              "observation of 400 (converted into the AMM prices) and a max observation change per",
              "update of 8 (also converted into the AMM prices). Observations can be updated once",
              "a minute, so 2% allows the proposal market to reach double the spot price or 0",
              "in 50 minutes."
            ],
            "type": "u128"
          },
          {
            "name": "twapMaxObservationChangePerUpdate",
            "type": "u128"
          },
          {
            "name": "twapStartDelaySlots",
            "docs": [
              "Forces TWAP calculation to start after amm.created_at_slot + twap_start_delay_slots"
            ],
            "type": "u64"
          },
          {
            "name": "minQuoteFutarchicLiquidity",
            "docs": [
              "As an anti-spam measure and to help liquidity, you need to lock up some liquidity",
              "in both futarchic markets in order to create a proposal.",
              "",
              "For example, for META, we can use a `min_quote_futarchic_liquidity` of",
              "5000 * 1_000_000 (5000 USDC) and a `min_base_futarchic_liquidity` of",
              "10 * 1_000_000_000 (10 META)."
            ],
            "type": "u64"
          },
          {
            "name": "minBaseFutarchicLiquidity",
            "type": "u64"
          },
          {
            "name": "seqNum",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "propLp",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "lpOwner",
            "type": "publicKey"
          },
          {
            "name": "stagedProp",
            "type": "publicKey"
          },
          {
            "name": "baseTokenContributed",
            "type": "u64"
          },
          {
            "name": "quoteTokenContributed",
            "type": "u64"
          },
          {
            "name": "rewardsRedeemed",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "stagedProposal",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "stagedId",
            "type": "u64"
          },
          {
            "name": "daoAddress",
            "type": "publicKey"
          },
          {
            "name": "proposer",
            "type": "publicKey"
          },
          {
            "name": "descriptionUrl",
            "type": "string"
          },
          {
            "name": "instruction",
            "type": {
              "defined": "StagedProposalInstruction"
            }
          },
          {
            "name": "stage",
            "type": {
              "defined": "StagedProposalState"
            }
          },
          {
            "name": "rewardTokens",
            "type": {
              "defined": "RewardTokens"
            }
          },
          {
            "name": "daoTokenThresholds",
            "type": {
              "defined": "DAOTokenThresholds"
            }
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "CreateStagedPropArgs",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "stagedId",
            "type": "u64"
          },
          {
            "name": "daoAddress",
            "type": "publicKey"
          },
          {
            "name": "descriptionUrl",
            "type": "string"
          },
          {
            "name": "instruction",
            "type": {
              "defined": "StagedProposalInstruction"
            }
          },
          {
            "name": "rewardTokens",
            "type": {
              "defined": "RewardTokens"
            }
          },
          {
            "name": "daoTokenThresholds",
            "type": {
              "defined": "DAOTokenThresholds"
            }
          }
        ]
      }
    },
    {
      "name": "FundStagedPropArgs",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "amount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "WithdrawFromStagedPropArgs",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "amount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "IxInitProposalArgs",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "nonce",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "IxInitializeQuestionArgs",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "proposalPubkey",
            "type": "publicKey"
          }
        ]
      }
    },
    {
      "name": "DAOTokenThresholds",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "baseMint",
            "type": "publicKey"
          },
          {
            "name": "quoteMint",
            "type": "publicKey"
          },
          {
            "name": "baseThreshold",
            "type": "u64"
          },
          {
            "name": "baseDecimals",
            "type": "u8"
          },
          {
            "name": "quoteThreshold",
            "type": "u64"
          },
          {
            "name": "quoteDecimals",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "RewardTokens",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "rewardTokenMint",
            "type": "publicKey"
          },
          {
            "name": "rewardTokenAmountBase",
            "type": "u64"
          },
          {
            "name": "rewardTokenAmountQuote",
            "type": "u64"
          },
          {
            "name": "rewardTokenDecimals",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "StagedProposalAccount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "pubkey",
            "type": "publicKey"
          },
          {
            "name": "isSigner",
            "type": "bool"
          },
          {
            "name": "isWritable",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "StagedProposalInstruction",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "programId",
            "type": "publicKey"
          },
          {
            "name": "accounts",
            "type": {
              "vec": {
                "defined": "StagedProposalAccount"
              }
            }
          },
          {
            "name": "data",
            "type": "bytes"
          }
        ]
      }
    },
    {
      "name": "StagedProposalState",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Staged"
          },
          {
            "name": "Market"
          },
          {
            "name": "Passed"
          },
          {
            "name": "Executed"
          },
          {
            "name": "Failed"
          },
          {
            "name": "Finalized"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "InvalidTokenMint",
      "msg": "Invalid token mint"
    },
    {
      "code": 6001,
      "name": "StagedPropAlreadyMarket",
      "msg": "Staged proposal already in market"
    },
    {
      "code": 6002,
      "name": "StagedPropNotLiquid",
      "msg": "Staged prop not liquid enough"
    },
    {
      "code": 6003,
      "name": "StagedPropStillFunding",
      "msg": "Cannot withdraw rewards if prop is still funding"
    }
  ]
};
