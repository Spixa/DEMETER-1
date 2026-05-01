## Distributed key-value store with BFT
This is a hobbyist project focused on creating a Byzantine fault-tolerant (BFT) distributed system over unreliable networks with quorum certificates, view changes and leader election designed for a fictitious facility that will use this distributed system for real-life applications.

### BFT advantages
In a volatile scenario like ours, we shouldn't rule out possibilities of not just node crashes, but also compromised nodes that can lie or equivocate (i.e. tell 2 different nodes a different story). So, using this BFT KV store for our purposes, will tolerate arbitrary malicious behaviour including lying, selective omission and equivocation.
#### Example attack
The compromised node sends `PREPARE` for some value on a key to peer `A` and a `PREPARE` with a different value for the same key to peer `B`. Using cryptography, namely signed message digests and quorum intersection, we can detect this equivocation


# Lore: DEMETER-1
DEMETER-1 (Deep Earth Magma Energy Transfer & Experimental Reactor) is a geothermal energy facility built within an extinct volcano caldera in an undisclosed location near the Pacific Ring of Fire operated by a consortium of 7 nations (US, Chile, Japan, Indonesia, Germany, South Korea, New Zealand). No single country fully trusts the others, but they must cooperate, because a catastrophic failure could possibly trigger a volcanic eruption affecting millions of lives near the facility. Based on where the facility is within the Ring of Fire, it could be lives of the citizens of one of the nations in the consortium, so the stakes are high. 

## Vision 
DEMETER-1, other than being a powerful power plant, is also a multi-national scientific and energy facility with 3 main co-located missions
### Mission 1: Magma-Thermal Energy Extraction
* 12 geothermal wells drilled 8 kilometres deep into the magma chamber inside the caldera
	* Supercritical $CO_2$ circulated to extract $500\;MW$ of base load power, enough for 360,000 homes
* Hydrogen electrolysis for green fuel

### Mission 2: Neutrino Observatory Extension
Scientists deemed the large expanse of this caldera coupled with its geographical location fit for their specific scientific observations. For this reason they will turn the caldera lake into an observatory:
* Fill the lake with $2km^3$ of ultra-pure water 
* Photomultiplier tubes on the lake bottom
* This observatory will detect supernovae and solar neutrinos (and possibly other cool exotic physics)
* Routes research data to external database

### Mission 3: Magma Laboratory
* Studies rheology of partially molten rock
* In-situ sensors inside the magma conduits
* Tomography with penetrating X-rays to capture the crystallisation process
* Routes research data to external database

## Infrastructure
Complete list of the key-values the nodes can commit in this distributed system
Also for reference, a *parameter* is a list of key-value pairs
#### The 4 zones
The facility is broken into 4 zones:
##### 1. The Magma conduit (`/magma/`)
Parameters: Flow velocity, Magma viscosity, Wall strain, Volatiles 
##### 2. The Heat exchanger farm (`/heat/`)
Parameters: Turbine inlet temperature, Bypass fraction, Cooling flow
##### 3. The Neutrino Observatory (`/neutrino/`)
Parameters: Photomultiplier tubes voltage, Trigger threshold, Data routing
##### 4. Emergency systems (`/emergency/`, required quorum is 6 out of 7)
Parameters: Magma quench system, Evacuation sirens system
## The governance model
As previously mentioned, in addition to being a 7 nation venture that could possibly be disagreeing in many matters, the project is intrinsically very volatile. So I propose a Byzantine Fault-Tolerant system that works like the following: Each nation has a control node in the BFT cluster. Critical decisions require 5 out of 7 signatures. By $2f+1$, we are tolerating 2 Byzantine nations (not literally lol)




