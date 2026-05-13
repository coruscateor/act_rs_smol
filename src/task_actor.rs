

use smol::{Executor, Task};

use act_rs::ActorStateAsync;

use crate::AutoDetachTask;

///
/// A task based actor.
/// 
pub struct TaskActor
{
}

impl TaskActor
{

    pub fn spawn<ST>(state: ST, ex: &Executor) -> AutoDetachTask<()>
        where ST: ActorStateAsync + Send + 'static
    {
        
        let task = ex.spawn(async move
        {
    
            TaskActor::run(state).await;

        });

        AutoDetachTask::new(task)

    }

    async fn run<ST>(mut state: ST)
        where ST: ActorStateAsync + Send + 'static
    {

        let mut proceed = true; 
        
        if state.pre_run_async().await
        {

            while proceed
            {
                
                proceed = state.run_async().await;
    
            }

        }
        
        state.post_run_async().await;

    }

}
