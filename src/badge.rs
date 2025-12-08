use crate::color::coverage_color;

/// Formats the percentage for display.
/// Shows decimal only if it's not a whole number.
fn format_percentage(percentage: f64) -> String {
    if percentage.fract() == 0.0 {
        format!("{}%", percentage as i64)
    } else {
        format!("{:.1}%", percentage)
    }
}

/// Generates an SVG badge for the given coverage percentage.
pub fn generate_badge(percentage: f64) -> String {
    let color = coverage_color(percentage);
    let percentage_text = format_percentage(percentage);

    // Approximate width calculation (shields.io style)
    // Label "coverage" is ~52px, percentage varies
    let label_width = 60;
    let value_width = 10 + (percentage_text.len() as i32 * 7);
    let total_width = label_width + value_width;
    let label_x = label_width / 2;
    let value_x = label_width + (value_width / 2);

    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"20\">
  <linearGradient id=\"smooth\" x2=\"0\" y2=\"100%\">
    <stop offset=\"0\" stop-color=\"#bbb\" stop-opacity=\".1\"/>
    <stop offset=\"1\" stop-opacity=\".1\"/>
  </linearGradient>
  <clipPath id=\"round\">
    <rect width=\"{}\" height=\"20\" rx=\"3\" fill=\"#fff\"/>
  </clipPath>
  <g clip-path=\"url(#round)\">
    <rect width=\"{}\" height=\"20\" fill=\"#555\"/>
    <rect x=\"{}\" width=\"{}\" height=\"20\" fill=\"{}\"/>
    <rect width=\"{}\" height=\"20\" fill=\"url(#smooth)\"/>
  </g>
  <g fill=\"#fff\" text-anchor=\"middle\" font-family=\"DejaVu Sans,Verdana,Geneva,sans-serif\" font-size=\"11\">
    <text x=\"{}\" y=\"15\" fill=\"#010101\" fill-opacity=\".3\">coverage</text>
    <text x=\"{}\" y=\"14\">coverage</text>
    <text x=\"{}\" y=\"15\" fill=\"#010101\" fill-opacity=\".3\">{}</text>
    <text x=\"{}\" y=\"14\">{}</text>
  </g>
</svg>",
        total_width,
        total_width,
        label_width,
        label_width,
        value_width,
        color,
        total_width,
        label_x,
        label_x,
        value_x,
        percentage_text,
        value_x,
        percentage_text
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_badge_contains_coverage_label() {
        let svg = generate_badge(85.0);
        assert!(
            svg.contains("coverage"),
            "Badge should contain 'coverage' label"
        );
    }

    #[test]
    fn test_badge_contains_percentage() {
        let svg = generate_badge(85.0);
        assert!(svg.contains("85%"), "Badge should contain percentage value");
    }

    #[test]
    fn test_badge_contains_correct_color() {
        let svg = generate_badge(85.0);
        assert!(svg.contains("#4c1"), "85% coverage should use green color");

        let svg_low = generate_badge(30.0);
        assert!(
            svg_low.contains("#e05d44"),
            "30% coverage should use red color"
        );
    }

    #[test]
    fn test_badge_is_valid_svg() {
        let svg = generate_badge(50.0);
        assert!(svg.starts_with("<svg"), "Should start with <svg");
        assert!(svg.ends_with("</svg>"), "Should end with </svg>");
    }

    #[test]
    fn test_percentage_formatting_whole_number() {
        let svg = generate_badge(85.0);
        assert!(svg.contains("85%"));
        assert!(!svg.contains("85.0%"));
    }

    #[test]
    fn test_percentage_formatting_decimal() {
        let svg = generate_badge(85.5);
        assert!(svg.contains("85.5%"));
    }

    #[test]
    fn test_format_percentage_whole_numbers() {
        assert_eq!(format_percentage(0.0), "0%");
        assert_eq!(format_percentage(50.0), "50%");
        assert_eq!(format_percentage(100.0), "100%");
    }

    #[test]
    fn test_format_percentage_decimals() {
        assert_eq!(format_percentage(50.5), "50.5%");
        assert_eq!(format_percentage(99.9), "99.9%");
        assert_eq!(format_percentage(0.1), "0.1%");
    }

    #[test]
    fn test_badge_yellow_coverage() {
        let svg = generate_badge(65.0);
        assert!(
            svg.contains("#dfb317"),
            "65% coverage should use yellow color"
        );
    }

    #[test]
    fn test_badge_boundary_values() {
        // Test boundary at 50%
        let svg_49 = generate_badge(49.9);
        assert!(svg_49.contains("#e05d44"), "49.9% should be red");

        let svg_50 = generate_badge(50.0);
        assert!(svg_50.contains("#dfb317"), "50% should be yellow");

        // Test boundary at 80%
        let svg_79 = generate_badge(79.9);
        assert!(svg_79.contains("#dfb317"), "79.9% should be yellow");

        let svg_80 = generate_badge(80.0);
        assert!(svg_80.contains("#4c1"), "80% should be green");
    }

    #[test]
    fn test_badge_dimensions() {
        let svg = generate_badge(50.0);
        assert!(svg.contains("height=\"20\""), "Badge should have height 20");
        assert!(svg.contains("width="), "Badge should have width attribute");
    }

    #[test]
    fn test_badge_has_gradient() {
        let svg = generate_badge(50.0);
        assert!(svg.contains("linearGradient"), "Badge should have gradient");
        assert!(
            svg.contains("id=\"smooth\""),
            "Gradient should have id 'smooth'"
        );
    }
}
