# XIRRCalc

XIRRCalc is a command-line interface tool designed to calculate the XIRR (Extended Internal Rate of Return) from cash flow data stored in a CSV file.

## Table of Contents

- [Features](#features)
- [Usage](#usage)
- [CSV File Format](#csv-file-format)
- [Support & Contribution](#support--contribution)
- [License](#license)

## Features

- Quick calculation of XIRR from a CSV file.
- Ability to specify custom date formats.
- Flexibility in date formatting with default support for `yyyy-mm-dd` and `dd/mm/yy`.

## Usage

```bash
$ ./xirrcalc [path-to-cashflows.csv] [date-format]
```

#### Arguments:

- [path-to-cashflows.csv] : The path to your CSV file containing the cash flows.
- [date-format] (optional) : Date format of the entries in your CSV. If not provided, the tool will try to recognize either yyyy-mm-dd or dd/mm/yy formats.

## CSV File Format

Ensure your CSV file conforms to the following format:

```
amount,date
1000,01/01/2022
-500,06/01/2022
```

## Support & Contribution

For any issues or feature requests, please open an issue or submit a pull request. Contributions are always welcome!

## License

This project is licensed under the MIT License. See the LICENSE file for details.
