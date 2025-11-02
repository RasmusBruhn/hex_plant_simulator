use std::sync::Arc;
use thiserror::Error;
use wgpu::BackendOptions;
use winit::{dpi::PhysicalSize, window::Window};

/// All the objects related to rendering including the device, command queue and surface
pub struct RenderState {
    /// The logical device connected to the gpu
    device: wgpu::Device,
    /// The command queue for sending info to the gpu
    queue: wgpu::Queue,
    /// The surface to draw on
    surface: wgpu::Surface<'static>,
    /// The configurations of the surface
    config: wgpu::SurfaceConfiguration,
}

impl RenderState {
    /// Creates a new render state from a given window
    ///
    /// # Parameters
    ///
    /// window: The window to use for the render state
    ///
    /// # Errors
    ///
    /// See NewRenderStateError for a description of the different errors which may occur
    pub async fn new(window: &Arc<Window>) -> Result<Self, NewRenderStateError> {
        // Get the size of the window
        let size = window.inner_size();
        if size.width <= 0 || size.height <= 0 {
            return Err(NewRenderStateError::InvalidSize(size));
        }

        // Get a handle to the API
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            flags: wgpu::InstanceFlags::VALIDATION,
            backend_options: BackendOptions::from_env_or_default(),
            memory_budget_thresholds: wgpu::MemoryBudgetThresholds::default(),
        });

        // Get a surface for the window
        let surface = instance.create_surface(window.clone())?;

        // Get an adapter to the GPU
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await?;

        // Create a logical device and a command queue
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("Request Device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: wgpu::MemoryHints::Performance,
                trace: wgpu::Trace::Off,
                experimental_features: wgpu::ExperimentalFeatures::default(),
            })
            .await?;

        // Get the capabilities of the surface
        let surface_caps = surface.get_capabilities(&adapter);

        // Get an sRGB texture format for the surface
        let surface_format = *surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .ok_or(NewRenderStateError::IncompatibleSurface)?;

        // Setup the configurations and configure the surface
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            desired_maximum_frame_latency: 2,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        Ok(Self {
            device,
            queue,
            surface,
            config,
        })
    }

    /// Called when the window has been resized
    ///
    /// # Parameters
    ///
    /// new_size: The new size of the window
    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);
    }

    /// Get a reference to the device
    pub fn get_device(&self) -> &wgpu::Device {
        &self.device
    }

    /// The a reference to the queue
    pub fn get_queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    /// Get a reference to the surface
    pub fn get_surface(&self) -> &wgpu::Surface<'_> {
        &self.surface
    }

    /// Get a reference to the configs
    pub fn get_config(&self) -> &wgpu::SurfaceConfiguration {
        &self.config
    }
}

/// The error types for when creating a new RenderState
#[derive(Error, Debug, Clone)]
pub enum NewRenderStateError {
    /// Either the width or the height of the supplied window were to small
    #[error("The width and height of the window must be larger than 0 but received {:?}", .0)]
    InvalidSize(PhysicalSize<u32>),
    /// The surface could not be created
    #[error("Unable to create surface: {:?}", .0)]
    CreateSurface(wgpu::CreateSurfaceError),
    /// The gpu adapter could not be created
    #[error("Unable to get adapter for gpu: {:?}", .0)]
    GetAdapter(wgpu::RequestAdapterError),
    /// The device and queue could not be created
    #[error("Unable to retrieve logical device: {:?}", .0)]
    RequestDevice(wgpu::RequestDeviceError),
    /// There was no comatible surface on the device
    #[error("No compatible surface found")]
    IncompatibleSurface,
}

impl From<wgpu::CreateSurfaceError> for NewRenderStateError {
    fn from(value: wgpu::CreateSurfaceError) -> Self {
        return Self::CreateSurface(value);
    }
}

impl From<wgpu::RequestDeviceError> for NewRenderStateError {
    fn from(value: wgpu::RequestDeviceError) -> Self {
        return Self::RequestDevice(value);
    }
}

impl From<wgpu::RequestAdapterError> for NewRenderStateError {
    fn from(value: wgpu::RequestAdapterError) -> Self {
        return Self::GetAdapter(value);
    }
}
