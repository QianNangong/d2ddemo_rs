#![windows_subsystem = "windows"]

use windows::{
    core::w,
    Foundation::Numerics::Matrix3x2,
    Win32::{
        Foundation::{D2DERR_RECREATE_TARGET, HINSTANCE, HWND, LPARAM, LRESULT, RECT, WPARAM},
        Graphics::{
            Direct2D::{
                Common::{D2D1_COLOR_F, D2D_POINT_2F, D2D_RECT_F, D2D_SIZE_U},
                D2D1CreateFactory, ID2D1Factory, ID2D1HwndRenderTarget, ID2D1SolidColorBrush,
                D2D1_FACTORY_TYPE_SINGLE_THREADED, D2D1_HWND_RENDER_TARGET_PROPERTIES,
                D2D1_RENDER_TARGET_PROPERTIES,
            },
            Gdi::{InvalidateRect, UpdateWindow, ValidateRect, COLOR_WINDOW, HBRUSH},
        },
        System::{
            SystemServices::IMAGE_DOS_HEADER,
            Threading::{GetStartupInfoW, STARTF_USESHOWWINDOW, STARTUPINFOW},
        },
        UI::WindowsAndMessaging::{
            CreateWindowExW, DefWindowProcW, DestroyWindow, DispatchMessageW, GetClientRect,
            GetMessageW, GetWindowLongPtrW, LoadImageW, MessageBoxW, PostQuitMessage,
            RegisterClassExW, SetWindowLongPtrW, ShowWindow, TranslateMessage, CS_HREDRAW,
            CS_VREDRAW, CW_USEDEFAULT, GWLP_USERDATA, HCURSOR, HICON, IDC_ARROW, IDI_APPLICATION,
            IMAGE_CURSOR, IMAGE_ICON, LR_DEFAULTSIZE, LR_SHARED, MB_ICONERROR, MB_OK, MSG,
            SHOW_WINDOW_CMD, SW_SHOWNORMAL, WINDOW_EX_STYLE, WM_CLOSE, WM_CREATE, WM_DESTROY,
            WM_DISPLAYCHANGE, WM_PAINT, WM_SIZE, WNDCLASSEXW, WS_OVERLAPPEDWINDOW,
        },
    },
};

extern "C" {
    static __ImageBase: IMAGE_DOS_HEADER;
}

#[inline]
fn get_instance_handle() -> HINSTANCE {
    unsafe { HINSTANCE(&__ImageBase as *const _ as isize) }
}

struct MountainApp {
    hwnd: HWND,
    factory: ID2D1Factory,
    render_target: Option<ID2D1HwndRenderTarget>,
    light_slate_gray_brush: Option<ID2D1SolidColorBrush>,
    cornflower_blue_brush: Option<ID2D1SolidColorBrush>,
}

impl MountainApp {
    fn new(hwnd: HWND) -> Result<Self, ::windows_core::Error> {
        let factory: ID2D1Factory =
            unsafe { D2D1CreateFactory(D2D1_FACTORY_TYPE_SINGLE_THREADED, None) }?;
        Ok(Self {
            hwnd,
            factory,
            render_target: None,
            light_slate_gray_brush: None,
            cornflower_blue_brush: None,
        })
    }

    fn create_device_resources(&mut self) -> Result<(), ::windows_core::Error> {
        if self.render_target.is_some() {
            return Ok(());
        }
        let render_target = unsafe {
            let mut rect = RECT::default();
            GetClientRect(self.hwnd, &mut rect)?;
            self.factory.CreateHwndRenderTarget(
                &D2D1_RENDER_TARGET_PROPERTIES::default(),
                &D2D1_HWND_RENDER_TARGET_PROPERTIES {
                    hwnd: self.hwnd,
                    pixelSize: D2D_SIZE_U {
                        width: (rect.right - rect.left) as u32,
                        height: (rect.bottom - rect.top) as u32,
                    },
                    ..Default::default()
                },
            )?
        };
        let light_slate_gray_brush = unsafe {
            render_target.CreateSolidColorBrush(
                &D2D1_COLOR_F {
                    r: 0.466666666666667,
                    g: 0.533333333333333,
                    b: 0.6,
                    a: 1.0,
                },
                None,
            )?
        };
        let cornflower_blue_brush = unsafe {
            render_target.CreateSolidColorBrush(
                &D2D1_COLOR_F {
                    r: 0.392156862745098,
                    g: 0.584313725490196,
                    b: 0.929411764705882,
                    a: 1.0,
                },
                None,
            )?
        };
        self.render_target = Some(render_target);
        self.light_slate_gray_brush = Some(light_slate_gray_brush);
        self.cornflower_blue_brush = Some(cornflower_blue_brush);
        Ok(())
    }

    fn discard_device_resources(&mut self) {
        self.render_target = None;
        self.light_slate_gray_brush = None;
        self.cornflower_blue_brush = None;
    }

    fn on_paint(&mut self) -> Result<(), ::windows_core::Error> {
        self.create_device_resources()?;
        if let (Some(render_target), Some(light_slate_gray_brush), Some(cornflower_blue_brush)) = (
            &self.render_target,
            &self.light_slate_gray_brush,
            &self.cornflower_blue_brush,
        ) {
            unsafe {
                render_target.BeginDraw();
                render_target.SetTransform(&Matrix3x2::identity());
                render_target.Clear(Some(&D2D1_COLOR_F {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    a: 1.0,
                }));
                let size = render_target.GetSize();
                for x in (0..(size.width as i32)).step_by(10) {
                    render_target.DrawLine(
                        D2D_POINT_2F {
                            x: x as f32,
                            y: 0.0,
                        },
                        D2D_POINT_2F {
                            x: x as f32,
                            y: size.height,
                        },
                        light_slate_gray_brush,
                        0.5,
                        None,
                    );
                }
                for y in (0..(size.height as i32)).step_by(10) {
                    render_target.DrawLine(
                        D2D_POINT_2F {
                            x: 0.0,
                            y: y as f32,
                        },
                        D2D_POINT_2F {
                            x: size.width,
                            y: y as f32,
                        },
                        light_slate_gray_brush,
                        0.5,
                        None,
                    );
                }
                render_target.FillRectangle(
                    &D2D_RECT_F {
                        left: size.width / 2.0 - 50.0,
                        top: size.height / 2.0 - 50.0,
                        right: size.width / 2.0 + 50.0,
                        bottom: size.height / 2.0 + 50.0,
                    },
                    light_slate_gray_brush,
                );
                render_target.FillRectangle(
                    &D2D_RECT_F {
                        left: size.width / 2.0 - 100.0,
                        top: size.height / 2.0 - 100.0,
                        right: size.width / 2.0 + 100.0,
                        bottom: size.height / 2.0 + 100.0,
                    },
                    cornflower_blue_brush,
                );
                match render_target.EndDraw(None, None) {
                    Err(e) if e.code() == D2DERR_RECREATE_TARGET => self.discard_device_resources(),
                    r => r?,
                };
            }
        }
        Ok(())
    }

    fn on_size(&self, width: u32, height: u32) -> Result<(), ::windows_core::Error> {
        if let Some(render_target) = &self.render_target {
            unsafe { render_target.Resize(&D2D_SIZE_U { width, height }) }
        } else {
            Ok(())
        }
    }
}

extern "system" fn window_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_CREATE => {
            let app = Box::new(MountainApp::new(hwnd).unwrap());
            unsafe {
                let app = Box::into_raw(app);
                SetWindowLongPtrW(hwnd, GWLP_USERDATA, app as isize);
            }
            LRESULT(0)
        }
        WM_CLOSE => {
            unsafe {
                let _result = DestroyWindow(hwnd);
            }
            LRESULT(0)
        }
        WM_DESTROY => unsafe {
            let app = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut MountainApp;
            let _boxed = Box::from_raw(app);
            PostQuitMessage(0);
            LRESULT(0)
        },
        WM_DISPLAYCHANGE => unsafe {
            InvalidateRect(hwnd, None, false);
            LRESULT(0)
        },
        WM_PAINT => unsafe {
            let app = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut MountainApp;
            let _result = app.as_mut().unwrap().on_paint();
            ValidateRect(hwnd, None);
            LRESULT(0)
        },
        WM_SIZE => unsafe {
            let (height, width) = (((lparam.0 >> 16) as i16) as u32, lparam.0 as i16 as u32);
            let app = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut MountainApp;
            let _result = app.as_mut().unwrap().on_size(width, height);
            LRESULT(0)
        },
        _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
    }
}

fn main() {
    let class_name = w!("Mountain");
    unsafe {
        if RegisterClassExW(&WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(window_proc),
            hInstance: get_instance_handle(),
            hIcon: HICON(
                LoadImageW(
                    None,
                    IDI_APPLICATION,
                    IMAGE_ICON,
                    0,
                    0,
                    LR_DEFAULTSIZE | LR_SHARED,
                )
                .unwrap()
                .0,
            ),
            hCursor: HCURSOR(
                LoadImageW(
                    None,
                    IDC_ARROW,
                    IMAGE_CURSOR,
                    0,
                    0,
                    LR_DEFAULTSIZE | LR_SHARED,
                )
                .unwrap()
                .0,
            ),
            hbrBackground: HBRUSH((COLOR_WINDOW.0 + 1) as isize),
            lpszClassName: class_name,
            ..Default::default()
        }) == 0
        {
            MessageBoxW(
                None,
                w!("Failed to register window class..."),
                w!("Error"),
                MB_ICONERROR | MB_OK,
            );
            return;
        }
    };
    let hwnd = unsafe {
        CreateWindowExW(
            WINDOW_EX_STYLE(0),
            class_name,
            w!("Mountain"),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            None,
            None,
            get_instance_handle(),
            None,
        )
    };
    if hwnd.0 == 0isize {
        unsafe {
            MessageBoxW(
                None,
                w!("Failed to create window..."),
                w!("Error"),
                MB_ICONERROR | MB_OK,
            )
        };
        return;
    }
    let mut si = STARTUPINFOW::default();

    unsafe { GetStartupInfoW(&mut si) };
    if !si.dwFlags.contains(STARTF_USESHOWWINDOW) {
        si.wShowWindow = SW_SHOWNORMAL.0 as u16;
    }
    unsafe {
        ShowWindow(hwnd, SHOW_WINDOW_CMD(si.wShowWindow as i32));
        UpdateWindow(hwnd);
    };
    let mut msg = MSG::default();
    while unsafe { GetMessageW(&mut msg, None, 0, 0).0 > 0 } {
        unsafe {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}
