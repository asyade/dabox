# Dabox

# Requirements

- Nightly rust toolchain
  > I'm curently using rustc 1.84.0-nightly (705cfe0e9 2024-11-01)

## Overview

In this assignment, you'll build a Dropbox-like system where users can manage a
hierarchical structure of directories and subdirectories. This system should
allow the creation, renaming, deletion, and listing of directories with infinite
nesting (subdirectories). You are expected to implement both a frontend and
backend for this feature.

Your solution should demonstrate good coding practices, system design, and a
focus on performance and scalability.

Requirements

1. API Build a RESTful API that supports the following directory management
   operations:

Create a Directory:

The directory should be created under the given parent directory. Rename a
Directory:

Renames the directory with the specified ID. Delete a Directory:

Deleting a directory should also remove all its subdirectories recursively. List
Directories:

This should return the directory structure starting from the given directory ID,
including all its subdirectories. 2. Frontend Build a simple interface that
allows users to:

View the directory structure: Display directories and subdirectories in a tree
format. Create a new directory: A form that allows users to input a directory
name and optionally select a parent directory. Rename a directory: Enable users
to rename directories via an inline edit or form. Delete a directory: Provide
the ability to delete a directory, with a confirmation step. The frontend should
reflect updates in real-time when a user creates, renames, or deletes a
directory.

It should be user-friendly, with minimal but effective design.

3. Additional Requirements Code quality: Solution should be production ready
   quality. Deployment: Please document how we can deploy this solution locally
   and in production environment.
4. Bonus Features (Optional) Provide a search feature to quickly find a
   directory by name. If you think your solution does not scale to 1 million
   users (which is fine), please document how you would address scalability
   concerns. Provide documentation on how to implement authentication and
   authorization Submission Guidelines Code: Provide a link to a GitHub
   repository or a zip file containing your code. Include instructions on how to
   set up and run the project locally. Documentation: Include a README with
   details about the solution, the design decisions you made, and how to run the
   project in production. Good luck!
