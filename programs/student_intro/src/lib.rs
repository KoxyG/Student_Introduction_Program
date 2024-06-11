use anchor_lang::prelude::*;

declare_id!("DW5zGn3TJdt1kPUrWnSuqjq8yG1iG83vDS3MpeCxfiH4");

#[program]
pub mod student_intro {
    use super::*;

    pub fn introduce(ctx: Context<Introduce>, name: String, message: String ) -> Result<()> {
        msg!("New student account created");
        msg!("Name: {}", name);
        msg!("Message: {}", message);

        let student_intro = &mut ctx.accounts.student_intro;
        student_intro.introducer = ctx.accounts.initializer.key();
        student_intro.name = name;
        student_intro.message = message;
        Ok(())
    }

    pub fn update_introduce(ctx: Context<UpdateIntroduce>, name: String, message: String,) -> Result<()> {
        msg!("Student introduction account space reallocated");
        msg!("Name: {}", name);
        msg!("Message: {}", message);

        let student_intro = &mut ctx.accounts.student_intro;
        student_intro.name = name;
        student_intro.message = message;
        Ok(())
    }

    pub fn delete_student_intro(_ctx: Context<DeleteStudentInro>, name: String) -> Result<()> {
        msg!("Student introduction account deleted: {}", name);
        Ok(())
    } 
}


#[derive(Accounts)]
#[instruction(name: String, message: String)]
pub struct Introduce<'info> {
    #[account(
        init,
        seeds = [name.as_bytes(), initializer.key().as_ref()],
        bump,
        payer = initializer,
        space = 8 + 32 + 4 + name.len() + 5 + message.len()
    )]
    pub student_intro: Account<'info, StudentAccountState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
#[instruction(name: String, message: String)]
pub struct UpdateIntroduce<'info> {
    #[account(
        mut,
        seeds = [name.as_bytes(), initializer.key().as_ref()],
        bump,
        realloc = 8 + 32 + 4 + name.len() + 4 + message.len(),
        realloc::payer = initializer,
        realloc::zero = true,
    )]
    pub student_intro: Account<'info, StudentAccountState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
#[instruction(name: String)]
pub struct DeleteStudentInro<'info> {
    #[account(
        mut,
        seeds=[name.as_bytes(), initializer.key().as_ref()],
        bump,
        close=initializer
    )]
    pub student_intro: Account<'info, StudentAccountState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct StudentAccountState {
    pub introducer: Pubkey, // 32
    pub name: String,       // 4 + len()
    pub message: String,    // 5 + len()
}

