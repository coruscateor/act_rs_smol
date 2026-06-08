commit e1fe413258fe6fb6e390c4433aadc7311f00008b -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Sun Jun 7 19:10:07 2026 +1200

    - Updated various dependences via “cargo update”.
    
    - Updated the reademe.
    
    - Made the run_catch_unwind method of the TaskActor implementation depend on the futures feature.

commit d036c90ec75862ed83787d0b47b1dc0e7d9a8374 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Sat Jun 6 18:35:23 2026 +1200

    - Updated the package version to 0.2.0-beta.
    
    - Prepared the changelog.
    
    - Added documentation
    
    - Added the spawn_and_build_state, spawn_catch_unwind, spawn_build_state_and_catch_unwind and run_catch_unwind methods to the TaskActor struct.
    
    - Other minor changes.

commit f5c1b7ddcd91e25ef632a5e5938f588685422d3e
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Fri Jun 5 17:21:56 2026 +1200

    - Updated the inc_dec dependency to version 0.2.0.

commit 886d1d5b509a4212f791db1bbb583f86a4381323
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Mon May 25 16:52:33 2026 +1200

    - Added the futures dependency.
    
    - Removed futures-lite from the thread_pool dependencies list.
    
    - Made the inclusion of the task_actor_macro_tests module partially dependant on the thread_pool feature.
    
    - Basically completed the test functions and related functionality in the task_actor_macro_tests module.
    
    - Fixed the impl_task_actor_build_state macro.
    
    - Made the ThreadPool implementation compatible with both the futures_lite and futures crates.

commit 3165711b195736e7cab4ab9030761ba2cd4357f1
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Sun May 24 19:39:24 2026 +1200

    - Added the task_actor_macro_tests module with TestActorState, TestActorStateBuilder, TestActorFlowState, TestActorFlowStateBuilder, TestPaincHander structs, without_builder and with_builder functions and task_actor, task_actor_build_state, task_actor_build_state_with_spawn, task_actor_flexible, task_actor_build_state_flexible, task_actor_build_state_with_spawn_flexible, task_actor_catch_unwind, task_actor_build_state_and_catch_unwind, task_actor_build_state_with_spawn_catch_unwind, task_actor_catch_unwind_flexible, task_actor_build_state_and_catch_unwind_flexible and task_actor_build_state_with_spawn_catch_unwind_flexible test functions.
    
    - Added the  impl_task_actor_build_state_with_spawn_catch_unwind, impl_task_actor_catch_unwind_flexible, impl_task_actor_build_state_and_catch_unwind_flexible and impl_task_actor_build_state_with_spawn_catch_unwind_flexible macros.

commit ce2cf7c430cccca59e80f30dbf02e72bbb1ef822
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Sat May 23 18:27:31 2026 +1200

    - Added the impl_task_actor_catch_unwind and impl_task_actor_build_state_and_catch_unwind macros.

commit 34977d80d914e01903feb39b2cb0dcfca1e79e5d
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Fri May 22 18:41:41 2026 +1200

    - Added “/.vscode” and “/old” strings to the .gitignore file.
    
    - Removed the .vscode directory.

commit 59d025b40aef84fbf848ae57f668927e6c8ca1ba
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Fri May 22 17:27:01 2026 +1200

    - Added the impl_task_actor_build_state_flexible and impl_task_actor_build_state_with_spawn_flexible macros.
    
    - Continued work on the ThreadPool.

commit 4b0121698a9b41928ab3cf78cd360ce7da304e87
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Thu May 21 17:38:11 2026 +1200

    - Added the accessorise and pastey optional dependencies and made them be included when you specify the thread_pool feature.
    
    - Added the impl_task_actor_build_state_with_spawn and impl_task_actor_flexible macros.
    
    - Continued work on the ThreadPool object.

commit 6faa86fbe60017cc2138c535e0de459f5f94040e
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Wed May 20 18:33:46 2026 +1200

    - Added an inc_dec optional dependency.
    
    - Added an futures-lite optional dependency.
    
    - Added a thread_pool feature.
    
    - Added a ThreadPool struct.

commit 2a2415cf5e97d315a24c5876b834e699637c31f8
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Mon May 18 19:03:04 2026 +1200

    - Uncommented package.metadata.docs.rs section in the Cargo.toml file.
    
    - Uncommented and changed “doc_auto_cfg” to “doc_cfg” in the docsrs package level cfg_attr decoration.
    
    - Renamed the mac_task_actors module to task_actor_macros.

commit eda43f9db8d909cadbc077a1402d69b9f1780b7d
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Wed May 13 13:09:38 2026 +1200

    - Set the act_rs dependency version to version 0.5.0.
    
    - Continued work on the AutoDetachTask struct.
    
    - Renamed the impl_mac_task_actor macro to impl_task_actor and updated its spawn meta-method to return an AutoDetachTask instance.
    
    - Removed the spawn_attached meta-method impl_task_actor of the macro.
    
    - Renamed impl_mac_task_actor_built_state to impl_task_actor_build_state and rearranged it to work like the impl_task_actor macro. Its spawn meta-method was renamed to spawn_and_build_state and it now returns an AutoDetachTask instance.
    
    - Removed the spawn_attached meta-method impl_task_actor_build_state of the macro.
    
    - The spawn method of the TaskActor implementation now returns an AutoDetachTask instance.
    
    - The spawn_attached method of the TaskActor implementation has been removed.

commit 65bf4d91a1644a040774c5a447b7c7f9fc9f0426
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Wed Mar 18 19:50:58 2026 +1300

    - Updated the package version string to "0.2.0-alpha".
    
    - Updated the act_rs dependency to point to a local repository.
    
    - Added the async-trait feature.
    
    - Made made the presence of the TaskActor struct dependant on the newly added async-trait feature.
    
    - Added the AutoDetachTask
