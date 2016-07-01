# DataCenterTopology

This is an updated repo to store the various working parts of the DataCenterTopology project.

The aim of this project is to programatically discover the physical topology of a series of machines in a data center in order to automatically, or with little administrative input, create a Ceph crushmap.

It is broken down into two parts: a single controller and a series of subordinate nodes which gather information for said controller. Due to the limitations with Juju charms the only sane method for communication between multiple nodes is in a node/server relationship.
