# frust

## about

frust is a tui application for making git-ops a less frustrating process.
frust assumes that you have your git-ops relevant files, for example kubernetes deployments and so on, bundled in a single repo separated from the code.
this means, especially with git-ops, that for a new version to be deployed, you have to manually update the deployment with a new image version.
frust takes this manual labour away, by offering you a intuitive tui, to work with your projects.

frust is not indented to run in a automated environment, for example in a jenkins pipeline. frust works right where you do, on your local machine.

