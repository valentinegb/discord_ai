use discord_ai::{ begin_training_history, respond_to_training_history, add_training_event };

#[tokio::test]
async fn training_mode() {
    begin_training_history("valentinegb (1234567890) sent a message in #general (1234567890): Hey Albert?").await;
    respond_to_training_history("Hello Valentine! How can I help?").await;
    add_training_event("valentinegb (1234567890) sent a message in #general (1234567890): Oh cool, it works!").await;
    add_training_event("valentinegb (1234567890) sent a message in #general (1234567890): Hmm... how do you say hello in German?").await;
    respond_to_training_history("Hello in German would be \"hallo\"!").await;
    add_training_event("valentinegb (1234567890) sent a message in #general (1234567890): That's funny, very similar to hello in English").await;
    respond_to_training_history("Yes, that is because English is a Germanic language.").await;
    add_training_event("valentinegb (1234567890) sent a message in #general (1234567890): Ohh yeah").await;
    add_training_event("valentinegb (1234567890) sent a message in #general (1234567890): I remember someone telling me that").await;
}