//! Publisher to publish messages.
//!
//! When creating publisher, you can specify a QoS profile.
//!
//! # Examples
//!
//! ## Default QoS Profile
//!
//! ```
//! use safe_drive::{context::Context, msg::common_interfaces::std_msgs};
//!
//! let ctx = Context::new().unwrap();
//! let node_pub = ctx
//!     .create_node("publisher_rs", None, Default::default())
//!     .unwrap();
//!
//! // Create a publisher.
//! let publisher = node_pub
//!     .create_publisher::<std_msgs::msg::UInt32>("publisher_rs_topic", None)
//!     .unwrap();
//!
//! // Send a message.
//! let mut msg = std_msgs::msg::UInt32::new().unwrap();
//! msg.data = 1234;
//! publisher.send(&msg).unwrap();
//! ```
//!
//! ## Specifying QoS Profile
//!
//! ```
//! use safe_drive::{context::Context, msg::common_interfaces::std_msgs, qos::{Profile, policy::HistoryPolicy}};
//!
//! let ctx = Context::new().unwrap();
//! let node_pub = ctx
//!     .create_node("publisher_rs_qos", None, Default::default())
//!     .unwrap();
//!
//! // Create a QoS policy.
//! let mut profile = Profile::default();
//! profile.history = HistoryPolicy::KeepAll;
//!
//! // Create a publisher with the QoS profile.
//! let publisher = node_pub
//!     .create_publisher::<std_msgs::msg::UInt32>("publisher_rs_topic_qos", Some(profile))
//!     .unwrap();
//! ```
//!
//! `None` of the 2nd argument of `create_publisher` is equivalent to `Some(Profile::default())`.

use crate::{
    error::{DynError, RCLResult},
    msg::TopicMsg,
    node::Node,
    qos, rcl,
};
use std::{ffi::CString, marker::PhantomData, ptr::null_mut, sync::Arc};

/// Publisher.
///
/// # Example
///
/// ```
/// use safe_drive::{context::Context, msg::common_interfaces::std_msgs};
///
/// let ctx = Context::new().unwrap();
/// let node_pub = ctx
///     .create_node("publish_rs", None, Default::default())
///     .unwrap();
///
/// // Create a publisher.
/// let publisher = node_pub
///     .create_publisher::<std_msgs::msg::UInt32>("publish_rs_topic", None)
///     .unwrap();
///
/// // Send a message.
/// let mut msg = std_msgs::msg::UInt32::new().unwrap();
/// msg.data = 1234;
/// publisher.send(&msg).unwrap();
/// ```
pub struct Publisher<T> {
    publisher: rcl::rcl_publisher_t,
    node: Arc<Node>,
    _phantom: PhantomData<T>,
}

impl<T: TopicMsg> Publisher<T> {
    pub(crate) fn new(
        node: Arc<Node>,
        topic_name: &str,
        qos: Option<qos::Profile>,
    ) -> RCLResult<Self> {
        let mut publisher = rcl::MTSafeFn::rcl_get_zero_initialized_publisher();

        let topic_name = CString::new(topic_name).unwrap_or_default();
        let options = Options::new(&qos.unwrap_or_default());

        {
            let guard = rcl::MT_UNSAFE_FN.lock();
            guard.rcl_publisher_init(
                &mut publisher,
                node.as_ptr(),
                T::type_support(),
                topic_name.as_ptr(),
                options.as_ptr(),
            )?;
        }

        Ok(Publisher {
            publisher,
            node,
            _phantom: Default::default(),
        })
    }

    /// Send a message.
    ///
    /// # Example
    ///
    /// ```
    /// use safe_drive::{context::Context, msg::common_interfaces::std_msgs};
    ///
    /// let ctx = Context::new().unwrap();
    /// let node = ctx
    ///     .create_node("publish_rs_send", None, Default::default())
    ///     .unwrap();
    ///
    /// // Create a publisher.
    /// let publisher = node.create_publisher("publish_rs_send_topic", None).unwrap();
    ///
    /// // Send a message.
    /// let msg = std_msgs::msg::Empty::new().unwrap();
    /// publisher.send(&msg).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// - `RCLError::InvalidArgument` if any arguments are invalid, or
    /// - `RCLError::PublisherInvalid` if the publisher is invalid, or
    /// - `RCLError::Error` if an unspecified error occurs.
    pub fn send(&self, msg: &T) -> Result<(), DynError> {
        if crate::is_halt() {
            return Err("Signaled".into());
        }
        rcl::MTSafeFn::rcl_publish(&self.publisher, msg as *const T as _, null_mut())?;
        Ok(())
    }
}

impl<T> Drop for Publisher<T> {
    fn drop(&mut self) {
        let (node, publisher) = (&mut self.node, &mut self.publisher);
        let guard = rcl::MT_UNSAFE_FN.lock();
        guard
            .rcl_publisher_fini(publisher, unsafe { node.as_ptr_mut() })
            .unwrap();
    }
}

/// Options for publishers.
struct Options {
    options: rcl::rcl_publisher_options_t,
}

impl Options {
    fn new(qos: &qos::Profile) -> Self {
        let options = rcl::rcl_publisher_options_t {
            qos: qos.into(),
            allocator: rcl::MTSafeFn::rcutils_get_default_allocator(),
            rmw_publisher_options: rcl::MTSafeFn::rmw_get_default_publisher_options(),
        };
        Options { options }
    }

    pub(crate) fn as_ptr(&self) -> *const rcl::rcl_publisher_options_t {
        &self.options
    }
}

unsafe impl<T> Sync for Publisher<T> {}
unsafe impl<T> Send for Publisher<T> {}
