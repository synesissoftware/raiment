fn main() {
    let ran = std::cell::Cell::new(false);

    {
        raiment::on_drop! {
            ran.set(true);
        }

        assert!(!ran.get());
    }

    assert!(ran.get());
}

/* ///////////////////////////// end of file //////////////////////////// */
