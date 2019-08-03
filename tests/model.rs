use ankurah::model::*;

#[test]
fn basic {
    let me = Agent::new();

    let wheel       = entity!("Wheel");
    let in_english  = entity!("English Words");
    let roundthings = entity!("Round Things");
    let round1d     = entity!("Things that are round in one dimension");
    let round1d     = entity!("Things that are round in one dimension");

    me.allege(wheel, in_english);
    me.allege(wheel, roundthings);
    me.allege(wheel, round1d);
    me.allege(wheel, roll);



    // Wheel is in the category of Wheel ( how to pluralize? )
    // Wheel is in the category of round things
    // Wheel is in the category of things that are round in one dimension
    // Wheel is in the category of things that roll
    // Roll is in the category of Roll
    // Roll is in the category of things a Wheel does
    // Roll is in the category of things that reduce friction



    // And & But are synonyms, except But is "Barbeque-flavored And"
    // And & But are both in the class of things that affirm the premise
    // "But" refines the premise, whereas "And" extends it? ( not really satisfying)


    // "I like turtles"


    // This context "a conversation on a park bench" is a member of:
    //     [ Uconversation is a member of [ Uoccurred is a member of [Date], Uattendees is a member of [] ] ]
    let context = entity!( "A conversation on a park bench", entity!("Conversation", entity!("Occurred", Date) );
     // TODO: how do I state who the conversation was "With" ?

    //me.allege(session)

    //      context, subject, predicate,         object (Label)
    entity!(context, daniel,    entity!("Said"), entity!("I like turtles") )
    // Daniel is a person (referring to a party within this conversation)
    let daniel = me.allege(session, entity!("Daniel"), entity!("Person"), None);

    // "I" is Daniel because Daniel is the one speaking
    let i = me.allege(session, daniel, "I", None);


    me.allege(session, i, like_turtles)
    entity!("I")

    assert
}