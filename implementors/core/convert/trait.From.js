(function() {var implementors = {};
implementors["futures"] = [{text:"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;T&gt; for <a class=\"enum\" href=\"futures/enum.Async.html\" title=\"enum futures::Async\">Async</a>&lt;T&gt;",synthetic:false,types:["futures::poll::Async"]},{text:"impl&lt;T, E&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;T, E&gt;&gt; for <a class=\"struct\" href=\"futures/future/struct.FutureResult.html\" title=\"struct futures::future::FutureResult\">FutureResult</a>&lt;T, E&gt;",synthetic:false,types:["futures::future::result_::FutureResult"]},{text:"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html\" title=\"struct alloc::sync::Arc\">Arc</a>&lt;T&gt;&gt; for <a class=\"struct\" href=\"futures/executor/struct.NotifyHandle.html\" title=\"struct futures::executor::NotifyHandle\">NotifyHandle</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"futures/executor/trait.Notify.html\" title=\"trait futures::executor::Notify\">Notify</a> + 'static,&nbsp;</span>",synthetic:false,types:["futures::task_impl::NotifyHandle"]},{text:"impl&lt;T:&nbsp;<a class=\"trait\" href=\"futures/executor/trait.Notify.html\" title=\"trait futures::executor::Notify\">Notify</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;'static </a>T&gt; for <a class=\"struct\" href=\"futures/executor/struct.NotifyHandle.html\" title=\"struct futures::executor::NotifyHandle\">NotifyHandle</a>",synthetic:false,types:["futures::task_impl::NotifyHandle"]},];
implementors["tokio_sync"] = [{text:"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\">(</a>T, TrySendError<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\">)</a>&gt; for <a class=\"struct\" href=\"tokio_sync/mpsc/error/struct.TrySendError.html\" title=\"struct tokio_sync::mpsc::error::TrySendError\">TrySendError</a>&lt;T&gt;",synthetic:false,types:["tokio_sync::mpsc::bounded::TrySendError"]},{text:"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\">(</a>T, TrySendError<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\">)</a>&gt; for <a class=\"struct\" href=\"tokio_sync/mpsc/error/struct.UnboundedTrySendError.html\" title=\"struct tokio_sync::mpsc::error::UnboundedTrySendError\">UnboundedTrySendError</a>&lt;T&gt;",synthetic:false,types:["tokio_sync::mpsc::unbounded::UnboundedTrySendError"]},];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
