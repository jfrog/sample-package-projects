# Cargo Package Manager Integration with Artifactory
A repo to demonstrate Rust's package manager, Cargo, and how it integrates with Artifactory.

# Project Goal
The two Rust projects in this folder will demonstrate the basics of integrating Rust's package manager, Cargo, with 
Jfrog's Artifactory.

# Project Structure
This repo will hold two projects: an "Inner Source" library, to be built and published with Artifactory's Cargo 
integration, and a "Production" application that will incorporate the library.

## Inner Source Library
A simple library that will return a randomly-selected quote from the late, great [Yogi Berra](https://en.wikipedia.org/wiki/Yogi_Berra)

## Production Application
A basic Rust application that will incorporate the Inner Source library.  


