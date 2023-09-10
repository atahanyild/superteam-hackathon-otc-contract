use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer as SplTransfer};
// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("CWVuSR4xskYr41ARq4CdwRZ8UrGVpQYCKMT3JFS81GHE");

#[program]
pub mod otc {
    use super::*;
    // // pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()> {
    // //     ctx.accounts.new_account.data = data;
    // //     msg!("Changed data to: {}!", data); // Message will show up in the tx logs
    // //     Ok(())
    // // }
    pub fn new_trade(
        ctx: Context<CreateTrade>,
        sale_contract: Pubkey,
        get_contract: Pubkey,
        sale_amount: u64,
        get_amount: u64,
    ) -> Result<()> {
        let trade = &mut ctx.accounts.trade;
        trade.creator = *ctx.accounts.creator.key;
        trade.sale_contract = sale_contract;
        trade.sale_amount = sale_amount;
        trade.get_contract = get_contract;
        trade.get_amount = get_amount;
        trade.is_active = true;
        msg!("Created trade succesfuly.");
        Ok(())
    }

    pub fn execute_trade(
        ctx: Context<ExecuteTrade>,
        sale_amount: u64,
        get_amount: u64,
    ) -> Result<()> {
        let trade = &mut ctx.accounts.trade;
        let destination = &ctx.accounts.buyer_ata;
        let source = &ctx.accounts.creator_ata;
        let token_program = &ctx.accounts.token_program;
        let authority = &ctx.accounts.creator;

        let creator_cpi_accounts = SplTransfer {
            from: source.to_account_info().clone(),
            to: destination.to_account_info().clone(),
            authority: authority.to_account_info().clone(),
        };
        let cpi_program = token_program.to_account_info();
        //trade işlemini gerçekleştir
        token::transfer(
            CpiContext::new(cpi_program, creator_cpi_accounts),
            sale_amount,
        )?;

        let buyer_cpi_accounts = SplTransfer {
            to: source.to_account_info().clone(),
            from: destination.to_account_info().clone(),
            authority: authority.to_account_info().clone(),
        };
        let cpi_program = token_program.to_account_info();
        //trade işlemini gerçekleştir
        token::transfer(CpiContext::new(cpi_program, buyer_cpi_accounts), get_amount)?;

        trade.is_active = false;
        msg!("Executed trade succesfuly.");
        Ok(())
    }
}

#[derive(Accounts)]

pub struct CreateTrade<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(init,payer=creator,seeds=[creator.key.as_ref()],space=32+32+32+32+8+8+8+1+8,bump)]
    pub trade: Account<'info, Trade>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]

pub struct ExecuteTrade<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    pub creator_ata: Account<'info, TokenAccount>,
    pub buyer: AccountInfo<'info>,
    pub buyer_ata: Account<'info, TokenAccount>,
    // #[account(mut)]
    pub trade: Account<'info, Trade>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Trade {
    pub creator: Pubkey,
    pub sale_contract: Pubkey,
    pub get_contract: Pubkey,
    pub sale_amount: u64,
    pub get_amount: u64,
    pub buyer: Option<Pubkey>,
    pub is_active: bool,
}
