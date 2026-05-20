use std::thread::{self, JoinHandle, Scope, ScopedJoinHandle, Thread, available_parallelism};

use std::io::Error;

use inc_dec::IncDecSelf;

use smol::{Executor, spawn};

use smol::channel::unbounded;

use futures_lite::future;

pub struct ThreadPool<'a>
{

    join_handles: Vec<ScopedJoinHandle<'a, ()>>

}

impl<'a> ThreadPool
{

    pub fn new(ex: Executor<'a>) -> Result<(), Error>
    {

        let avalible_parallelism_res = available_parallelism();

        let (signal, shutdown) = unbounded::<()>();

        match avalible_parallelism_res
        {

            Ok(val) =>
            {

                let join_handles = thread::scope(|scope| //: &'a Scope<'a, '_>| //<'a, _, Vec<ScopedJoinHandle<'a, ()>>
                {

                    let mut number_of_threads: usize = val.into();

                    let mut new_join_handles = Vec::with_capacity(number_of_threads);

                    while number_of_threads > 0
                    {

                        let sjh = scope.spawn(||
                        { 
                            
                            future::block_on(ex.run(shutdown.recv())); 
                        
                        });

                        /*
                        let jh = thread::spawn(||
                        { 
                            
                            future::block_on(ex.run(shutdown.recv())); 
                        
                        });
                        */

                        new_join_handles.push(sjh);

                        number_of_threads.mm();

                    }

                    new_join_handles

                });




                Ok(())

            }
            Err(err) =>
            {

                Err(err)

            }
        }

    }

}