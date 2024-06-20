// Copyright (C) 2024 FusionSuite Team
//
// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU Affero General Public License as published by the Free
// Software Foundation, version 3.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more
// details. You should have received a copy of the GNU Affero General Public
// License along with this program. If not, see <https://www.gnu.org/licenses/>.

pub fn clean_serial(serial: String) -> String {
    match serial.to_lowercase().as_str() {
        "to be filled by o.e.m." => "".to_string(),
        "not specified" => "".to_string(),
        "none" => "".to_string(),
        _ => serial
    }
}

pub fn clean_string(data: String) -> String {
    match data.to_lowercase().as_str() {
        "unknown" => "".to_string(),
        "not specified" => "".to_string(),
        _ => data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_serial1() {
        let data = "To Be Filled By O.E.M.".to_string();
        let result = clean_serial(data);
        let expected = "".to_string();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_clean_serial2() {
        let data = "To Be Filled By O.E.M".to_string();
        let result = clean_serial(data);
        let expected = "To Be Filled By O.E.M".to_string();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_clean_serial3() {
        let data = "XX6hty".to_string();
        let result = clean_serial(data);
        let expected = "XX6hty".to_string();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_clean_serial4() {
        let data = "".to_string();
        let result = clean_serial(data);
        let expected = "".to_string();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_clean_string1() {
        let data = "Unknown".to_string();
        let result = clean_string(data);
        let expected = "".to_string();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_clean_string2() {
        let data = "unknown".to_string();
        let result = clean_string(data);
        let expected = "".to_string();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_clean_string3() {
        let data = "XX6hty".to_string();
        let result = clean_string(data);
        let expected = "XX6hty".to_string();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_clean_string4() {
        let data = "".to_string();
        let result = clean_string(data);
        let expected = "".to_string();

        assert_eq!(result, expected);
    }

}