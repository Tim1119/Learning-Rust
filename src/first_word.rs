pub fn get_first_word(x:&str)->&str{

    for (i,j) in x.bytes().enumerate(){
        if j == b' '{
            return &x[0..i]
        }
    }

    &x[..]
}