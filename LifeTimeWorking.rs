struct Holder {
    i: i32
}

fn held(h: &Holder) -> &i32 {
    &h.i
}
