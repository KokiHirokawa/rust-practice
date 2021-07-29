use std::slice;
use std::str;

fn main() {

    // - SeeAlso: https://doc.rust-lang.org/std/primitive.str.html

    // strは文字列スライスと呼ばれている
    // スライスとは、メモリ上に存在する文字列データのスタート地点と長さを示しているにすぎない
    // そのため、扱える文字列データは固定サイズで変更もできない

    {
        let hello_world = "Hello, World!";
        println!("{}", hello_world);
    }

    {
        let hello_world: &'static str = "Hello, World!";
        println!("{}", hello_world);
    }

    {
        static HELLO_WORLD: &str = "Hello, World!";
        println!("{}", HELLO_WORLD);
    }

    {
        let story = "Once upon a time...";

        let ptr = story.as_ptr();
        let len = story.len();

        assert_eq!(19, len);

        // unsafeによりメモリ安全性が保証されない領域に足を踏み入れることができる
        let s = unsafe {
            let slice = slice::from_raw_parts(ptr, len);
            str::from_utf8(slice)
        };

        assert_eq!(s, Ok(story));
    }

    {
        let len = "foo".len();
        assert_eq!(3, len); // 3文字ではなく、3bytes
    }

    {
        let word = "goodbye";

        let count = word.chars().count();
        assert_eq!(7, count);

        let mut chars = word.chars();

        assert_eq!(Some('g'), chars.next());
        assert_eq!(Some('o'), chars.next());
        assert_eq!(Some('o'), chars.next());
        assert_eq!(Some('d'), chars.next());
        assert_eq!(Some('b'), chars.next());
        assert_eq!(Some('y'), chars.next());
        assert_eq!(Some('e'), chars.next());
        assert_eq!(None, chars.next());
    }

    {
        let s = "";
        assert!(s.is_empty());
    }

    {
        let s = "A few words";
        let mut iter = s.split_whitespace();

        assert_eq!(Some("A"), iter.next());
        assert_eq!(Some("few"), iter.next());
        assert_eq!(Some("words"), iter.next());
        assert_eq!(None, iter.next());
    }

    {
        let text = "foo\r\nbar\n\nbaz\n";
        let mut lines = text.lines();

        assert_eq!(Some("foo"), lines.next());
        assert_eq!(Some("bar"), lines.next());
        assert_eq!(Some(""), lines.next());
        assert_eq!(Some("baz"), lines.next());
        assert_eq!(None, lines.next());
    }
}