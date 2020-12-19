mod tests
{

extern crate modules_crates;

#[test]
//#[should_panic]
//#[ignore]
fn english_greeting_correct(){
    assert_eq!("hello", modules_crates::greetings::english::hello());
}

}