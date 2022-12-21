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



declare_id!("BhH3yyskVj5UTC1jKSjAYYEzu29JfYU2gA3dtukMq7ar");

#[program]
pub mod myepicproject { 
  use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;

use super::*;
  pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result <()> {
    let base_account = &mut ctx.accounts.base_account;

    base_account.total_games = 0;
    base_account.total_gameswon = 0;
    base_account.total_gameslost = 0;

  
      Ok(())
    
  }
 

  pub fn dep_bet(ctx: Context<DepBet>, bet_amount: String) -> Result <()> {
    let my_string = bet_amount.to_string();  // `parse()` works with `&str` and `String`!

    let betamountconverted= my_string.parse::<u64>().unwrap();
    let feescovered = betamountconverted * 1025 / 1000 ;
    let base_account = &mut ctx.accounts.base_account;


    let clock = Clock::get()?;
    let coinflip_result = clock.unix_timestamp % 10; // any number between range 0-10 
    let  lamports2 = feescovered * LAMPORTS_PER_SOL;



     let transfer_instruction = &transfer(
       ctx.accounts.user.to_account_info().key,
       base_account.to_account_info().key,
       lamports2,
   );

//    let transfer_instructionwon = &transfer(
//     base_account.to_account_info().key,
//     ctx.accounts.user.to_account_info().key,
//     lamportswon,
// );


   invoke(
    transfer_instruction,
    &[
        ctx.accounts.user.to_account_info(),
        base_account.to_account_info(),

    ]
)
.unwrap();


    if coinflip_result < 4 {
        base_account.total_gameswon += 1;
        let mut lamportswon : u64 = betamountconverted * 2;
      //   invoke(
      //     transfer_instructionwon,
      //     &[
      //         base_account.to_account_info(),
      //         ctx.accounts.user.to_account_info(),

      //     ]
      // )
      // .unwrap();
      base_account.bet_value = betamountconverted * 2;
    } else {
        base_account.total_gameslost += 1;
       let  lamportswon: u64 = betamountconverted * 0;
       base_account.bet_value = betamountconverted * 0;
    }






   

    base_account.total_games += 1;
    Ok(())
  }

// Claiming Rewards 

pub fn claim_rewards(ctx: Context<FlipCoin>) -> Result <()> {
      
   Ok(())
     
    }
}




#[derive(Accounts)]
pub struct StartStuffOff<'info> {
  #[account(init, payer = user, space = 300,)]

  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
  pub system_program: Program <'info, System>,
}

#[derive(Accounts)]
 pub struct DepositTreasury<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
  pub system_program: Program <'info, System>,
 }


#[derive(Accounts)]
pub struct DepBet<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
  pub system_program: Program <'info, System>,
}

#[derive(Accounts)]
pub struct FlipCoin<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
  pub system_program: Program <'info, System>,
  #[account(init ,  
    // State account seed uses the string "state" and the users' key. 
  // Note that we can only have 1 active transaction
  seeds = [b"escrow".as_ref(), from.key().as_ref(), to.key().as_ref()],
  bump,
  payer = from,
  space = size_of::<EscrowAccount>() + 16)]
  pub wallet_escrow: Account<'info, EscrowAccount>, 

  #[account(mut)]
  pub from: Signer<'info>,
  /// CHECK: safe
  #[account(mut)]
  pub to: AccountInfo<'info>,

}



#[account]
pub struct EscrowAccount {
    // From address
    pub from: Pubkey,

    // To address
    pub to: Pubkey,

    // Amount that is owed
    pub amount: u64,
}

#[account]
pub struct BaseAccount {
    pub total_games: u64,
    pub total_gameswon: u64,
    pub total_gameslost: u64,
    pub bet_value: u64,
    authority: Pubkey
}

