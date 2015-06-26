/^\s*unimplemented!()$/ { # if the current line is an "unimplemented!()"
    : isunimp
    N; # load next line
    /\n\s*unimplemented!()$/ { # see if that one is also "unimplemented!()"
        D; # delete first line, keep all the rest
        b isunimp
    }
}
