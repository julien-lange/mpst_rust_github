////////////////////////////////////////////
/// CLOSE

///  Create the close function to be used with more than 3 participants.
///  
///  # Arguments
///  
///  * The name of the new *close* function
///  * The name of the *SessionMpst* type that will be used
///  * The number of participants (all together)
///  
///  # Example
///  
///  ```
///  use mpstthree::{close_mpst, create_sessionmpst};
///
///  create_sessionmpst!(SessionMpst, 3);
///
///  close_mpst!(close_mpst_multi, SessionMpst, 3);
///  ```
#[macro_export]
macro_rules! close_mpst {
    ($func_name:ident, $struct_name:ident, $nsessions:literal) => {
        mpst_seq::seq!(N in 1..$nsessions {
            fn $func_name<R>(s: $struct_name<#(mpstthree::binary::End,)0:0 mpstthree::role::end::RoleEnd, R>) -> Result<(), Box<dyn std::error::Error>>
            where
                R: mpstthree::role::Role,
            {
                #(
                    s.session#N:0.sender.send(()).unwrap_or(());
                )0:0

                #(
                    s.session#N:0.receiver.recv()?;
                )0:0

                Ok(())
            }
        });
    }
}
