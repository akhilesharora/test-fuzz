mod debug {
    use serde::{Deserialize, Serialize};
    use std::fmt::{Debug, Formatter, Result};

    #[derive(Clone, Deserialize, Serialize)]
    struct Struct;

    impl Debug for Struct {
        fn fmt(&self, _f: &mut Formatter<'_>) -> Result {
            panic!("bug");
        }
    }

    #[test_fuzz::test_fuzz]
    fn target(s: &Struct) {}

    #[test]
    fn test() {
        target(&Struct);
    }
}
