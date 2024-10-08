# The ARK Protocol

## Overview

The ARK Protocol introduces a dynamic, flexible, and governance-agnostic framework known as Para Autonomous Organizations (PAOs), designed to enable seamless governance transitions, interactions, and collaborations between on-chain organizations. Unlike traditional governance models, ARK provides a flexible solution where organizations can migrate between governance systems without losing their on-chain history or data.

ARK ensures modularity, interoperability, and flexibility in governance, enabling both Decentralized Autonomous Organizations (DAOs) and Centralized Autonomous Organizations (CAOs) to interact in a unified, scalable ecosystem. The ARK Program acts as the global record-keeper and governance registry for all PAOs within the ARK network, ensuring that organizations operate independently while securely interacting with each other through the Standard.

## Key Concepts

- **PAOs (Para Autonomous Organizations)**: Autonomous entities that can adapt their governance framework while maintaining their data and state. PAOs can interact and transition between governance structures, ensuring flexibility.
- **SAOs (Sub-Autonomous Organizations)**: Organizations that rely on another PAO for governance decisions. They can be fully or partially dependent on a parent PAO.

## ARK Program Components

1. **ARK Program**: The central record-keeper for PAO states and interactions. It registers new PAO programs and manages state transitions.
2. **PAO Programs**: Independent governance frameworks within the ARK ecosystem. Each PAO program interacts with governance accounts to manage its own organization's structure.
3. **Standard Extensions**: Extensions are supplementary programs used for specific tasks such as cross-PAO communication, escrow management, and governance transitioning.

## Features

- **Modularity**: PAOs are modular units within the ARK Program that allow organizations to manage their governance without needing to deploy new programs.
- **Flexibility**: PAOs support governance transitions, allowing organizations to change their governance structures without losing data or operational context.
- **Interoperability**: PAOs communicate and interact via the ARK Standard Program, ensuring seamless collaboration across organizations.
- **Governance-Agnosticism**: Organizations can shift between decentralized and centralized governance models, making the system adaptable to various real-world scenarios. The ARK Protocol enables organizations to operate under different governance structures without being tied to a single framework.

## How to Contribute

We welcome contributions from developers who want to extend or build new PAO programs using the ARK Protocol. Here's how you can contribute:

### 1. Clone the Repository

First, fork and clone the repository to your local development environment:

```bash
git clone https://github.com/arkonsol/ark_protocol.git
cd ark_protocol
```

### 2. Set Up Anchor

Ensure you have the Anchor framework installed for developing Solana programs. If you haven't installed it yet, use:

```bash
cargo install --git https://github.com/coral-xyz/anchor --tag v0.30.1 anchor-cli --locked
```

### 3. Create a New Para-Autonomous Organization Program

To create a new PAO program, use the Anchor CLI to initialize a new program:

```bash
anchor new <program_name>
```

For example:

```bash
anchor new meritocracy
```

This creates a new folder for the program in your project, with the basic structure needed to build the PAO.

### 4. Set Up Program Structure

After initializing the program, navigate to your program folder and create the necessary subdirectories:

```bash
cd programs/<program_name>
```

Set up the following folders:

- `contexts`: To define the instruction handlers and program logic.
- `states`: To manage and define the on-chain accounts and data.
- `error.rs`: To define custom errors specific to your PAO program.

Example structure:

```bash
programs/
  meritocracy/
    src/
      contexts/
      states/
      error.rs
      lib.rs
```

### 5. Define the Program

Modify the `lib.rs` file to include your PAO logic. Be sure to define your instruction handlers, account structures, and error handling.

Example Instruction Handler:

```rust
use anchor_lang::prelude::*;
use the_ark_program::cpi::accounts::{RegisterGovernment, AddTokenToTreasury, CreateTreasury};
use the_ark_program::program::TheArkProgram;
use the_ark_program::state::analytics::ArkAnalytics;
use the_ark_program::cpi::register_government;
use the_ark_program::instructions::register_state::StateInfo;
use the_ark_program::instructions::register_state::GovernmentType;

#[program]
pub mod meritocracy {
    use super::*;

        pub fn initialize_and_register_government(ctx: Context<Initialize>, name: String) -> Result<()> {
        // Create CPI context
        let cpi_program = ctx.accounts.ark_program.to_account_info();
        let cpi_accounts = RegisterGovernment {
            payer: ctx.accounts.creator.to_account_info(),
            ark_analytics: ctx.accounts.ark_analytics.to_account_info(),
            state_info: ctx.accounts.state_info.to_account_info(),
            government_program: ctx.accounts.government_program.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        // Make the CPI call
        register_government(cpi_ctx, name, GovernmentType::Republic)?;
    
        Ok(())
    }

}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(mut)]
    pub ark_analytics: Account<'info, ArkAnalytics>,
    #[account(mut)]
    pub state_info: Account<'info, StateInfo>,
    /// CHECK: This is the program ID of the specific government type
    pub government_program: UncheckedAccount<'info>,
    pub ark_program: Program<'info, TheArkProgram>,
    pub system_program: Program<'info, System>,
}
```

### 6. Build and Test Your Program

Build your program to check for errors:

```bash
anchor build
```

After building, run tests to ensure everything works as expected:

```bash
anchor test
```

### 7. Deploy to Devnet

Once you've built and tested your program, deploy it to Devnet:

```bash
anchor deploy --provider.cluster devnet
```

### 8. Submit a Pull Request

Once you've made your contributions and tested the code, submit a pull request to the repository for review.

## How to Report Issues

If you encounter any issues while using the ARK Protocol or building PAO programs, feel free to open an issue in the repository. Provide as much detail as possible, including:

- Steps to reproduce the issue.
- Relevant error messages or logs.
- Your environment setup (e.g., OS, Solana version, Anchor version).

## License

This project is licensed under the Apache License Version 2.0. See the [LICENSE](./LICENSE). file for details.
