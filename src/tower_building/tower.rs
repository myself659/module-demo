
// absolute path, starting at the crate's root
use crate::auxiliary_building::plug::device as from_crate;

// relative path, navigating up through the module tree
use super::super::auxiliary_building::plug::device as with_super;
// we didn't create any other symbols in our small example, however,
// we can still resolve the other symbols we created with the upper
// two use expressions, as, for example, with_super. We introduce a
// new name for it (with_self) and are able to use it as well.
use self::with_super as with_self;

fn  do_it() {
    println!("tower do it")
}
pub fn start_consumption(){
    println!("start_consumption");
    // absolute path
    from_crate::do_it();

    // relative paths
    with_super::do_it();
    with_self::do_it();
}