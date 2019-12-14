(function() {var implementors = {};
implementors["futures"] = [];
implementors["lazy_queue"] = [{"text":"impl&lt;Item, P, I&gt; <a class=\"trait\" href=\"futures/future/trait.Future.html\" title=\"trait futures::future::Future\">Future</a> for <a class=\"struct\" href=\"lazy_queue/sync/bounded/struct.QueueProcessor.html\" title=\"struct lazy_queue::sync::bounded::QueueProcessor\">QueueProcessor</a>&lt;Item, P, I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(Item) -&gt; I,<br>&nbsp;&nbsp;&nbsp;&nbsp;I: <a class=\"trait\" href=\"futures/future/trait.IntoFuture.html\" title=\"trait futures::future::IntoFuture\">IntoFuture</a>,&nbsp;</span>","synthetic":false,"types":["lazy_queue::sync::bounded::QueueProcessor"]},{"text":"impl&lt;Item, P, I&gt; <a class=\"trait\" href=\"futures/future/trait.Future.html\" title=\"trait futures::future::Future\">Future</a> for <a class=\"struct\" href=\"lazy_queue/sync/unbounded/struct.QueueProcessor.html\" title=\"struct lazy_queue::sync::unbounded::QueueProcessor\">QueueProcessor</a>&lt;Item, P, I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(Item) -&gt; I,<br>&nbsp;&nbsp;&nbsp;&nbsp;I: <a class=\"trait\" href=\"futures/future/trait.IntoFuture.html\" title=\"trait futures::future::IntoFuture\">IntoFuture</a>,&nbsp;</span>","synthetic":false,"types":["lazy_queue::sync::unbounded::QueueProcessor"]},{"text":"impl&lt;Item, P, I&gt; <a class=\"trait\" href=\"futures/future/trait.Future.html\" title=\"trait futures::future::Future\">Future</a> for <a class=\"struct\" href=\"lazy_queue/unsync/bounded/struct.QueueProcessor.html\" title=\"struct lazy_queue::unsync::bounded::QueueProcessor\">QueueProcessor</a>&lt;Item, P, I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(Item) -&gt; I,<br>&nbsp;&nbsp;&nbsp;&nbsp;I: <a class=\"trait\" href=\"futures/future/trait.IntoFuture.html\" title=\"trait futures::future::IntoFuture\">IntoFuture</a>,&nbsp;</span>","synthetic":false,"types":["lazy_queue::unsync::bounded::QueueProcessor"]},{"text":"impl&lt;Item, P, I&gt; <a class=\"trait\" href=\"futures/future/trait.Future.html\" title=\"trait futures::future::Future\">Future</a> for <a class=\"struct\" href=\"lazy_queue/unsync/unbounded/struct.QueueProcessor.html\" title=\"struct lazy_queue::unsync::unbounded::QueueProcessor\">QueueProcessor</a>&lt;Item, P, I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(Item) -&gt; I,<br>&nbsp;&nbsp;&nbsp;&nbsp;I: <a class=\"trait\" href=\"futures/future/trait.IntoFuture.html\" title=\"trait futures::future::IntoFuture\">IntoFuture</a>,&nbsp;</span>","synthetic":false,"types":["lazy_queue::unsync::unbounded::QueueProcessor"]}];
implementors["tokio_sync"] = [{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"futures/future/trait.Future.html\" title=\"trait futures::future::Future\">Future</a> for <a class=\"struct\" href=\"tokio_sync/oneshot/struct.Receiver.html\" title=\"struct tokio_sync::oneshot::Receiver\">Receiver</a>&lt;T&gt;","synthetic":false,"types":["tokio_sync::oneshot::Receiver"]}];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        })()