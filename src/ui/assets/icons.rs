use objc2::msg_send;
use objc2::rc::Retained;
use objc2_core_foundation::CGPoint;
use objc2_ui_kit::UIBezierPath;

/// Returns the path for the success icon (Shield + Checkmark)
pub fn success_path() -> Retained<UIBezierPath> {
    let path = UIBezierPath::bezierPath();

    // Shield (Curved sides)
    path.moveToPoint(CGPoint::new(4.0, 4.0));
    path.addLineToPoint(CGPoint::new(16.0, 4.0));
    unsafe {
        let _: () = msg_send![
            &path,
            addCurveToPoint: CGPoint::new(10.0, 18.0),
            controlPoint1: CGPoint::new(16.0, 11.0),
            controlPoint2: CGPoint::new(13.0, 16.0)
        ];
        let _: () = msg_send![
            &path,
            addCurveToPoint: CGPoint::new(4.0, 4.0),
            controlPoint1: CGPoint::new(7.0, 16.0),
            controlPoint2: CGPoint::new(4.0, 11.0)
        ];
    }

    // Checkmark (Centered and balanced)
    path.moveToPoint(CGPoint::new(7.0, 10.0));
    path.addLineToPoint(CGPoint::new(9.5, 12.5));
    path.addLineToPoint(CGPoint::new(13.5, 7.5));
    path
}

/// Returns the path for the error icon (Triangle + Exclamation)
pub fn error_path() -> Retained<UIBezierPath> {
    let path = UIBezierPath::bezierPath();
    // Triangle
    path.moveToPoint(CGPoint::new(10.0, 3.0));
    path.addLineToPoint(CGPoint::new(18.0, 17.0));
    path.addLineToPoint(CGPoint::new(2.0, 17.0));
    path.closePath();

    // Exclamation
    path.moveToPoint(CGPoint::new(10.0, 7.0));
    path.addLineToPoint(CGPoint::new(10.0, 12.0));

    // Dot
    path.moveToPoint(CGPoint::new(10.0, 14.5));
    path.addLineToPoint(CGPoint::new(10.0, 14.5));
    path
}

/// Returns the path for the info icon (Hexagon + 'i')
pub fn info_path() -> Retained<UIBezierPath> {
    let path = UIBezierPath::bezierPath();
    // Hexagon
    path.moveToPoint(CGPoint::new(10.0, 2.0));
    path.addLineToPoint(CGPoint::new(17.0, 6.0));
    path.addLineToPoint(CGPoint::new(17.0, 14.0));
    path.addLineToPoint(CGPoint::new(10.0, 18.0));
    path.addLineToPoint(CGPoint::new(3.0, 14.0));
    path.addLineToPoint(CGPoint::new(3.0, 6.0));
    path.closePath();

    // Info 'i'
    // Dot
    path.moveToPoint(CGPoint::new(10.0, 6.0));
    path.addLineToPoint(CGPoint::new(10.0, 6.0));

    // Line
    path.moveToPoint(CGPoint::new(10.0, 9.0));
    path.addLineToPoint(CGPoint::new(10.0, 14.0));
    path
}

/// Returns the path for the menu icon (Hamburger)
pub fn menu_path() -> Retained<UIBezierPath> {
    let path = UIBezierPath::bezierPath();

    // Top bar
    path.moveToPoint(CGPoint::new(4.0, 12.0));
    path.addLineToPoint(CGPoint::new(20.0, 12.0));

    // Middle bar
    path.moveToPoint(CGPoint::new(4.0, 6.0));
    path.addLineToPoint(CGPoint::new(20.0, 6.0));

    // Bottom bar
    path.moveToPoint(CGPoint::new(4.0, 18.0));
    path.addLineToPoint(CGPoint::new(20.0, 18.0));

    path
}
