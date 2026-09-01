#[macro_export]
macro_rules! impl_identifier {
    ($($t:ty)+$(,)?) => {
        $(
            impl HasId for $t{
                fn alloc_id(&mut self){
                    self.id = Identifier(IdPool::with_pool<$t, _>(|pool| pool.next()));
                }

                fn free_id(&mut self){
                    IdPool::with_pool<$t, _>(|pool| pool.free(self.id.0));
                }
            }
        )+
    };
}