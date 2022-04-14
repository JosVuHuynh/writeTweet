use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod my_solana_program {
    use super::*;
    pub fn setup_platform(ctx: Context<TweetPlatform>) -> ProgramResult{
        let tweet = &mut ctx.accounts.tweet;
        tweet.likes = 0;
        tweet.message = ("").to_string();
        Ok(())
    }

    pub fn write_tweet(
        ctx: Context<WriteTweet>, 
        message: String, 
        user_public_key: Pubkey
    ) -> ProgramResult {
        let tweet = &mut ctx.accounts.tweet;

        if !tweet.message.trim().is_empty() {
            return Err(Errors::CannotUpdateTweet.into());
        }

        if message.trim().is_empty() {
            return Err(Errors::EmptyMessage.into());
        }

        tweet.message = message;
        tweet.likes = 0;
        tweet.creator = user_public_key;

        Ok(())
    }

    pub fn like_tweet(ctx: Context<LikeTweet>, user_linking_tweet: Pubkey) -> ProgramResult {
        let tweet = &mut ctx.accounts.tweet;
        
        if tweet.message.trim().is_empty() {
            return Err(Errors::NotValidTweet.into());
        }

        if tweet.likes == 5 {
            return Err(Errors::ReachedMaxLikes.into());
        }
        
        let mut iter = tweet.people_who_liked.iter();
        if iter.any(|&v| v == user_linking_tweet) {
            return Err(Errors::UserLikedTweet.into());
        }

        tweet.likes +=1;
        tweet.people_who_liked.push(user_linking_tweet);

        Ok(())
    }
}

#[derive(Accounts)] //Implements an Accounts deserializer on the given struct, applying any constraints specified via inert #[account(..)] attributes upon deserialization.
pub struct TweetPlatform<'info> {
    #[account(init, payer = user)] //Marks the account as being initialized, skipping the account discriminator check. When using init, a rent Sysvar must be present in the Accounts struct.
    pub tweet: Account<'info, Tweet>,

    #[account(mut)] //Marks the account as mutable and persists the state transition.
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account] //representing a Solana account
#[derive(Default)]
pub struct Tweet {
    message: String,
    likes: u8,
    creator: Pubkey,
    people_who_liked: Vec<Pubkey>,
}

#[derive(Accounts)]
pub struct WriteTweet<'info> {
    #[account(mut)]
    pub tweet: Account<'info, Tweet>,
}

#[derive(Accounts)]
pub struct LikeTweet<'info> {
    #[account(mut)]
    pub tweet: Account<'info, Tweet>
}



#[error]
pub enum Errors {
    #[msg("Tweet message cannot be updated")]
    CannotUpdateTweet,

    #[msg("Message cannot be empty")]
    EmptyMessage,

    #[msg("Cannot recieve mor than 5 likes")]
    ReachedMaxLikes,

    #[msg("Cannot like a tweet without a valid message")]
    NotValidTweet,

    #[msg("User has already liked the tweet")]
    UserLikedTweet,
}

