use anchor_lang::prelude::*;

declare_id!("E2xY5XEm4zuWUWhCGQ4CYWUEdDrbp5H4NHaZQHWSG8df");

#[program]
pub mod smacofl {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        freelancer: String,
        deadline: u8,
        payment: u64,
        milestones: u8,
    ) -> Result<()> {
        let work_contract = &mut ctx.accounts.contract;
        work_contract.hirer = ctx.accounts.signer.key.clone();
        work_contract.freelancer = freelancer.clone();
        work_contract.deadline = deadline.clone();
        work_contract.payment = payment.clone();
        work_contract.is_done = false;
        work_contract.milestones = milestones;
        work_contract.one_milestone_value = payment / milestones as u64 + 3;
        work_contract.done_milestones = 0;
        work_contract.is_active = true;
        msg!(
            "new contract between {} and  is succesfuly created.",
            work_contract.hirer,
            //freelancer
        );
        Ok(())
    }

    pub fn complete(ctx: Context<Complete>) -> Result<()> {
        use super::*;
        let work_contract = &mut ctx.accounts.contract;
        let from = ctx.accounts.from.to_account_info();
        let to = ctx.accounts.to.to_account_info();
        let done_milestones = work_contract.done_milestones;
        let milestones = work_contract.milestones;
        let amount_of_lamports = work_contract.one_milestone_value * 3;

        msg!("in complete");
        if milestones == done_milestones {
            msg!("contract is completed");
            work_contract.is_done = true;
            work_contract.is_active = false;
            **from.try_borrow_mut_lamports()? -= amount_of_lamports;
            **to.try_borrow_mut_lamports()? += amount_of_lamports;
        }

        Ok(())
    }

    pub fn complete_milestone(ctx: Context<CompleteMilestone>) -> Result<()> {
        let work_contract = &mut ctx.accounts.contract;
        let from = ctx.accounts.from.to_account_info();
        let to = ctx.accounts.to.to_account_info();
        let amount_of_lamports = work_contract.one_milestone_value;

        msg!("a new milestone has been complated");
        **from.try_borrow_mut_lamports()? -= amount_of_lamports;
        **to.try_borrow_mut_lamports()? += amount_of_lamports;
        work_contract.done_milestones += 1;

        Ok(())
    }

    pub fn cancel(ctx: Context<Cancel>) -> Result<()> {
        //who cancels pays %15 to the other
        let work_contract = &mut ctx.accounts.contract;
        let from = ctx.accounts.from.to_account_info();
        let to = ctx.accounts.to.to_account_info();
        let amount_of_lamports = work_contract.payment * 15 / 100;

        msg!("canceled contract");
        **from.try_borrow_mut_lamports()? -= amount_of_lamports;
        **to.try_borrow_mut_lamports()? += amount_of_lamports;
        work_contract.is_active = false;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init,seeds=[signer.key.as_ref()],bump,payer=signer,space=16+256)]
    pub contract: Account<'info, Work_contract>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Complete<'info> {
    #[account(mut,seeds=[signer.key.as_ref()],bump)]
    pub contract: Account<'info, Work_contract>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub from: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub to: AccountInfo<'info>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

//tekrarbak
#[derive(Accounts)]
pub struct CompleteMilestone<'info> {
    #[account(mut,seeds=[signer.key.as_ref()],bump)]
    pub contract: Account<'info, Work_contract>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub from: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub to: AccountInfo<'info>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
//tekrarbak
#[derive(Accounts)]
pub struct Cancel<'info> {
    #[account(mut,seeds=[signer.key.as_ref()],bump)]
    pub contract: Account<'info, Work_contract>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub from: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub to: AccountInfo<'info>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Work_contract {
    hirer: Pubkey,
    freelancer: String,
    deadline: u8,
    payment: u64,
    milestones: u8,
    done_milestones: u8,
    one_milestone_value: u64,
    is_done: bool,
    is_active: bool,
}
