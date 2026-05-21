use std::any::Any;
use std::num::NonZero;
use std::sync::Arc;
use std::thread::{self, JoinHandle, Scope, ScopedJoinHandle, Thread, available_parallelism};

use std::io::Error;

use accessorise::impl_get_ref;

use inc_dec::IncDecSelf;

use smol::{Executor, spawn};

use smol::channel::{Sender, unbounded};

use futures_lite::future;

use pastey::paste;

pub struct ThreadPool //<'a>
{

    join_handles: Vec<JoinHandle<()>>,
    _signal: Sender<()>,
    is_joining: bool

}

impl ThreadPool //<'a>
{

    pub fn new(arc_ex: &Arc<Executor<'static>>) -> Result<Self, Error>
    {

        let avalible_parallelism_res = available_parallelism();

        match avalible_parallelism_res
        {

            Ok(val) =>
            {

                let thread_pool = Self::with_threads(arc_ex, val);

                Ok(thread_pool)

                /*
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
                */

            }
            Err(err) =>
            {

                Err(err)

            }
        }

    }

    pub fn with_threads(arc_ex: &Arc<Executor<'static>>, number: NonZero<usize>) -> Self
    {

        let (_signal, shutdown) = unbounded::<()>();

        let join_handles = {
            
        let mut number_of_threads: usize = number.into();

        let mut new_join_handles = Vec::with_capacity(number_of_threads);
            
            while number_of_threads > 0
            {

                let ex_moved = arc_ex.clone(); 

                let shutdown_moved = shutdown.clone();

                let jh = thread::spawn(move ||
                { 
                    
                    let _ = future::block_on(ex_moved.run(shutdown_moved.recv())); 
                
                });

                new_join_handles.push(jh);

                number_of_threads.mm();

            }

            new_join_handles

        };

        Self { join_handles, _signal, is_joining: false }

    }

    pub fn new_and_executor() -> (Result<Self, Error>, Arc<Executor<'static>>)
    {

        let ex: Executor<'static> = Executor::new();

        let arc_ex = Arc::new(ex);

        let res = Self::new(&arc_ex);

        (res, arc_ex)

    }

    pub fn with_threads_and_executor(number: NonZero<usize>) -> (Self, Arc<Executor<'static>>)
    {

        let ex: Executor<'static> = Executor::new();

        let arc_ex = Arc::new(ex);

        let res = Self::with_threads(&arc_ex, number);

        (res, arc_ex)

    }

    pub fn number_of_threads(&self) -> usize
    {

        let len = self.join_handles.len();

        if self.is_joining
        {

            len + 1

        }
        else
        {

            len
            
        }

    }

    impl_get_ref!(join_handles, Vec<JoinHandle<()>>);

    pub fn join(mut self) -> Vec<Result<(), Box<dyn Any + Send>>>
    {

        self.is_joining = true;
        
        let mut join_results = Vec::new();

        for join_handle in self.join_handles.drain(..)
        {

            join_results.push(join_handle.join());

        }

        join_results

    }

    pub fn take_join_handles(self) -> Vec<JoinHandle<()>>
    {

        self.join_handles

    }

}

/*
impl Drop for ThreadPool
{

    fn drop(&mut self)
    {
        
        self.signal.send(());

    }

}
*/
