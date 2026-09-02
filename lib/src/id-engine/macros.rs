/// Macro used to implement the Identifier-Generation logic to a struct.
#[macro_export]
macro_rules! impl_identifier {
    ($($t:ty)+$(,)?) => {
        $(
            impl HasId for $t{
                fn alloc_id() -> Identifier{
                    Identifier(IdPool::with_pool::<$t, _>(|pool| pool.next()))
                }

                fn free_id(&mut self){
                    IdPool::with_pool::<$t, _>(|pool| pool.free(self.id.0));
                }
            }
        )+
    };
}
