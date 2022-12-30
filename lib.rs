use anchor_lang::accounts::program_account::ProgramAccount;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;
use anchor_lang::system_program;
use anchor_lang::solana_program::{clock, program_option::COption, sysvar};
use anchor_lang::solana_program::{
    lamports,
    program::{invoke, invoke_signed},
    system_instruction::{transfer , assign_with_seed, assign}
};
use std::mem::size_of;
declare_id!("ACgSoUuNYuS1MLrMpHuyrnsX1BzDLtMaUEEJyDvVBb7M");

#[program]
pub mod thegamerust {
    use std::string;


    use anchor_lang::Bump;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }


    pub fn create_escrow(ctx: Context<CreateEscrow>, amount: String) -> Result<()> {
        let my_string = amount.to_string();  // `parse()` works with `&str` and `String`!

        let betamountconverted= my_string.parse::<u64>().unwrap();
        let betamountlamp = betamountconverted * LAMPORTS_PER_SOL / 100; 
        let feeamount = betamountlamp * 25 / 1000;
        


    // Get Escrow Account
    // let escrow = &mut ctx.accounts.escrow;
    
    // // Set from
    // escrow.from = ctx.accounts.from.key();
    // // Set to
    // escrow.to = ctx.accounts.to.key();
    // // set amount
    // escrow.amount = betamountconverted;

// This 

let transferbet = anchor_lang::solana_program::system_instruction::transfer(
    &ctx.accounts.from.key(),
    &ctx.accounts.to.key(),
    betamountlamp,
);
anchor_lang::solana_program::program::invoke(
    &transferbet,
    &[
        ctx.accounts.from.to_account_info(),
        ctx.accounts.to.to_account_info(),
    ],
);

   //   TRANSFERING FEE TO WALLET
    let transferfees = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.from.key(),
        &ctx.accounts.fees.key(),
        feeamount,
    );
    anchor_lang::solana_program::program::invoke(
        &transferfees,
        &[
            ctx.accounts.from.to_account_info(),
            ctx.accounts.fees.to_account_info(),
        ],
    );

    


    // Calling of winner or loser 


    let clock = Clock::get()?;
    let coinflip_result = clock.unix_timestamp % 10;
    msg!("Tossing Coin");
    msg!("Game Result Number (For dev purposes): {}", coinflip_result);
    if coinflip_result >= 5 {
        let amountwon = betamountlamp * 2; 
        msg!("User has won");

    
        // let userpda= &mut(ctx.accounts.account_user);
        // let ix = anchor_lang::solana_program::system_instruction::transfer(

        //     &ctx.accounts.from.key(),
        //     &userpda.key(),
        //     amountwon,
        // );

        //     anchor_lang::solana_program::program::invoke(
        //         &ix,
        //         &[
        //             ctx.accounts.from.to_account_info(),
        //             userpda.to_account_info()
        //         ],
        //     );
        
        





    } else {
        let amountlost = betamountconverted * 0;
        msg!("User has lost"); 
    }



    Ok(())

    }



}










#[derive(Accounts)]
pub struct Initialize<'info>{
    #[instruction(from: Pubkey,)]
    #[account(
        init, 
        payer = to, 
        space = 100,
        seeds = [b"escrow", to.key().as_ref(), from.key().as_ref()], bump
    )]
    pub escrow: Account<'info, CoinFlip>,
    /// CHECK: safe
    #[account(mut)]
    pub to:  Signer<'info>,
    /// CHECK: safe
    #[account(mut)]
    pub from: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    #[account(mut)]
     /// CHECK: Not dangerous
    pub fees: AccountInfo<'info>,
    // #[account(init,seeds=[from.key.as_ref()],bump,payer=from,space=80)]
    // pub account_user:   Account<'info,AccountUser>,

}




// ESCROW 
#[derive(Accounts)]
pub struct CreateEscrow<'info> {
    #[account(
        mut, 
        seeds = [b"escrow", to.key().as_ref(), from.key().as_ref()], bump
    )]
    pub escrow: Account<'info, CoinFlip>,
    #[account(mut)]
    pub from: Signer<'info>,
    /// CHECK: safe
    #[account(mut)]
    pub to: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
    #[account(mut)]
     /// CHECK: Not dangerous
    pub fees: AccountInfo<'info>,
    // #[account(init,seeds=[from.key.as_ref()],bump,payer=from,space=80)]
    // pub account_user:   Account<'info,AccountUser>,


}








#[account]
pub struct EscrowAccount {
    // From address
    pub from: Pubkey,

    // To address
    pub to: Pubkey,
    
    // Amount that is owed
    pub amount: u64,
    pub gameresult: u64,
    
}

#[account]

pub struct FeeStatistics {
    gamesplayed: u64,
    totalfeescollected: u64
}

#[account]
pub struct AccountUser {
    pub user: String,
        }

#[account]
#[derive(Default)] 
pub struct CoinFlip {
    players: [Pubkey; 2], 
    vendor_seed: i64,
    bet_amount: u64,
    bump: u8
}