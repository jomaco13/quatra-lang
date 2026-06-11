use quatra::qud::Qud;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qud_creation() {
        assert_eq!(Qud::from_u8(0), Some(Qud::Zero));
        assert_eq!(Qud::from_u8(1), Some(Qud::One));
        assert_eq!(Qud::from_u8(2), Some(Qud::Super));
        assert_eq!(Qud::from_u8(3), Some(Qud::Error));
        assert_eq!(Qud::from_u8(4), None);
    }

    #[test]
    fn test_qnot() {
        assert_eq!(Qud::Zero.qnot(), Qud::One);
        assert_eq!(Qud::One.qnot(), Qud::Super);
        assert_eq!(Qud::Super.qnot(), Qud::Error);
        assert_eq!(Qud::Error.qnot(), Qud::Zero);
    }

    #[test]
    fn test_qmax() {
        assert_eq!(Qud::Zero.qmax(Qud::One), Qud::One);
        assert_eq!(Qud::One.qmax(Qud::Super), Qud::Super);
        assert_eq!(Qud::Error.qmax(Qud::One), Qud::Error);
    }

    #[test]
    fn test_qmin() {
        assert_eq!(Qud::Zero.qmin(Qud::One), Qud::Zero);
        assert_eq!(Qud::One.qmin(Qud::Super), Qud::One);
        assert_eq!(Qud::Error.qmin(Qud::One), Qud::One);
    }

    #[test]
    fn test_add() {
        assert_eq!(Qud::Zero + Qud::One, Qud::One);
        assert_eq!(Qud::One + Qud::One, Qud::Super);
        assert_eq!(Qud::Super + Qud::One, Qud::Error);
        assert_eq!(Qud::Error + Qud::One, Qud::Zero);
    }

    #[test]
    fn test_mul() {
        assert_eq!(Qud::Zero * Qud::One, Qud::Zero);
        assert_eq!(Qud::One * Qud::One, Qud::One);
        assert_eq!(Qud::Super * Qud::One, Qud::Super);
        assert_eq!(Qud::Error * Qud::One, Qud::Error);
    }

    #[test]
    fn test_not_trait() {
        assert_eq!(!Qud::Zero, Qud::One);
        assert_eq!(!Qud::One, Qud::Super);
    }

    #[test]
    fn test_add_trait() {
        assert_eq!(Qud::Zero + Qud::Zero, Qud::Zero);
        assert_eq!(Qud::Super + Qud::Super, Qud::Zero);
    }

    #[test]
    fn test_mul_trait() {
        assert_eq!(Qud::Super * Qud::Super, Qud::Zero);
        assert_eq!(Qud::Error * Qud::One, Qud::Error);
    }
}
