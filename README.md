# This is a simple Utility to Erase Free Space as per a Procedure
This is a simple utility that tries to clear the free space on a device by
creating an ever expanding file using the method defined by a procedure until
the operating system responds with an out of space error.

## Implemented Procedures
- [CSE-ITSG-06](https://www.cse-cst.gc.ca/en/node/270/html/10572)

## Supported Operating Systems
- unix (macOS, linux, etc)

## Contributions
Contributions are welcome, if you have a procedure you want to implement or
improve the tool please open an issue first to discuss it before submitting code.

## DISCLAIMER
This utility does not provide a 100% guarantee to data erasure located in areas
of the device marked as free space by the filesystem managing the device. This
is a best effort and in the event that absolute data erasure is necessary
ensure that the device is encrypted before storing the sensitive data and destroy
the device when it is decommissioned. This tool is more of a way to clean the
free space on a device that is still going to remain operational and you need to
preserve the content that is actually taking up space on the device. This
utility has not been verified by security critics / professionals so use at your
own discretion.
