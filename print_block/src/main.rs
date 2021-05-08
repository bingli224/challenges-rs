
// source: https://www.facebook.com/photo?fbid=1995477270630474&set=gm.1671508526373588

fn main() {
    println!("{}", "Programming" );
    println!("{}", into_square_shape("Programming".to_string()));
    println!("{}", "Programm" );
    println!("{}", into_square_shape("Programm".to_string()));
}

fn into_square_shape ( txt: String ) -> String {
    let mut output = String::new ( );
    
    let len = txt.char_indices ( ).count ( );

    for idx_y in 0 .. len {
        for idx_x in 0 .. len {
            if idx_y == 0 || idx_x == 0 {
                if idx_x > idx_y {
                    output.push ( txt.chars ( ).nth ( idx_x ).unwrap ( ) );
                } else {
                    output.push ( txt.chars ( ).nth ( idx_y ).unwrap ( ) );
                }
            } else if idx_x == len - 1 || idx_y == len - 1 {
                if idx_x > idx_y {
                    output.push ( txt.chars ( ).nth ( len - idx_y - 1 ).unwrap ( ) );
                } else {
                    output.push ( txt.chars ( ).nth ( len - idx_x - 1 ).unwrap ( ) );
                }
            } else if idx_x == idx_y || len - idx_x - 1 == idx_y {
                if idx_y >= len / 2 {
                    output.push ( txt.chars ( ).nth ( len - idx_x - 1 ).unwrap ( ) );
                } else {
                    output.push ( txt.chars ( ).nth ( idx_x ).unwrap ( ) );
                }
            } else {
                output += " ";
            }
            output += " ";
        }
        //* output.as_mut_str().get_mut ( len - 1.. ).unwrap ( ) = "\n";
        output.replace_range ( output.len() - 1.., "\n" );
    }

    output
}

#[test]
fn given_odd_len_string ( ) {
    let input = "Programming".to_string ( );
    let expect = String::new ( ) +
    "P r o g r a m m i n g\n" +
    "r r               n n\n" +
    "o   o           i   i\n" +
    "g     g       m     m\n" +
    "r       r   m       m\n" +
    "a         a         a\n" +
    "m       m   r       r\n" +
    "m     m       g     g\n" +
    "i   i           o   o\n" +
    "n n               r r\n" +
    "g n i m m a r g o r P\n"
    ;
    
    assert_eq ! ( into_square_shape(input), expect );
}

#[test]
fn given_even_len_string ( ) {
    let input = "Programm".to_string ( );
    let expect = String::new ( ) +
    "P r o g r a m m\n" +
    "r r         m m\n" +
    "o   o     a   a\n" +
    "g     g r     r\n" +
    "r     r g     g\n" +
    "a   a     o   o\n" +
    "m m         r r\n" +
    "m m a r g o r P\n"
    ;
    
    assert_eq ! ( into_square_shape(input), expect );
}