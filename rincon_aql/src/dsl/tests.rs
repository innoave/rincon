
use hamcrest::prelude::*;

use super::*;

#[test]
fn for_c_in_characters() {
    let query = Aql::for_("c").in_("Characters").return_("c");

    assert_that!(&query.to_string(), is(equal_to("FOR c IN Characters RETURN c")));
}
