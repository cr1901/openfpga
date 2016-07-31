/***********************************************************************************************************************
 * Copyright (C) 2016 Andrew Zonenberg and contributors                                                                *
 *                                                                                                                     *
 * This program is free software; you can redistribute it and/or modify it under the terms of the GNU Lesser General   *
 * Public License as published by the Free Software Foundation; either version 2.1 of the License, or (at your option) *
 * any later version.                                                                                                  *
 *                                                                                                                     *
 * This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied  *
 * warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for     *
 * more details.                                                                                                       *
 *                                                                                                                     *
 * You should have received a copy of the GNU Lesser General Public License along with this program; if not, you may   *
 * find one here:                                                                                                      *
 * https://www.gnu.org/licenses/old-licenses/lgpl-2.1.txt                                                              *
 * or you may search the http://www.gnu.org website for the version 2.1 license, or you may write to the Free Software *
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA                                      *
 **********************************************************************************************************************/

#include "gp4par.h"

using namespace std;

void MakeNetlistEdges(Greenpak4Netlist* netlist);
void MakeDeviceEdges(Greenpak4Device* device);

void MakeSingleNode(
	string type,
	Greenpak4BitstreamEntity* entity,
	PARGraph* ngraph,
	PARGraph* dgraph,
	labelmap& lmap);
	
PARGraphNode* MakeNode(
	uint32_t label,
	Greenpak4BitstreamEntity* entity,
	PARGraph* dgraph);

/**
	@brief Build the graphs
 */
void BuildGraphs(
	Greenpak4Netlist* netlist,
	Greenpak4Device* device,
	PARGraph*& ngraph,
	PARGraph*& dgraph,
	labelmap& lmap)
{
	//Create the graphs
	ngraph = new PARGraph;
	dgraph = new PARGraph;
	
	//This is the module being PAR'd
	Greenpak4NetlistModule* module = netlist->GetTopModule();
	
	//Create device entries for the IOBs
	uint32_t ibuf_label = AllocateLabel(ngraph, dgraph, lmap, "GP_IBUF");
	uint32_t obuf_label = AllocateLabel(ngraph, dgraph, lmap, "GP_OBUF");
	uint32_t iobuf_label = AllocateLabel(ngraph, dgraph, lmap, "GP_IOBUF");
	for(auto it = device->iobbegin(); it != device->iobend(); it ++)
	{
		auto iob = it->second;
		
		//Type A (and not input-only)? Can be anything
		if( (dynamic_cast<Greenpak4IOBTypeA*>(iob) != NULL) && !iob->IsInputOnly() )
		{
			auto node = MakeNode(iobuf_label, iob, dgraph);
			node->AddAlternateLabel(obuf_label);
			node->AddAlternateLabel(ibuf_label);
		}
		
		//Not input only, but type B? OBUF or IBUF but can't be IOBUF
		else if(!iob->IsInputOnly())
		{
			auto node = MakeNode(obuf_label, iob, dgraph);
			node->AddAlternateLabel(ibuf_label);
		}
		
		//Nope, just an input
		else
			MakeNode(ibuf_label, iob, dgraph);
	}
	
	//Make device nodes for each type of LUT
	uint32_t lut2_label = AllocateLabel(ngraph, dgraph, lmap, "GP_2LUT");
	uint32_t lut3_label = AllocateLabel(ngraph, dgraph, lmap, "GP_3LUT");
	uint32_t lut4_label = AllocateLabel(ngraph, dgraph, lmap, "GP_4LUT");
	for(unsigned int i=0; i<device->GetLUT2Count(); i++)
		MakeNode(lut2_label, device->GetLUT2(i), dgraph);
	for(unsigned int i=0; i<device->GetLUT3Count(); i++)
	{
		auto node = MakeNode(lut3_label, device->GetLUT3(i), dgraph);
		node->AddAlternateLabel(lut2_label);
	}
	for(unsigned int i=0; i<device->GetLUT4Count(); i++)
	{
		auto node = MakeNode(lut4_label, device->GetLUT4(i), dgraph);
		node->AddAlternateLabel(lut2_label);
		node->AddAlternateLabel(lut3_label);
	}
	
	//Make device nodes for the inverters
	uint32_t inv_label  = AllocateLabel(ngraph, dgraph, lmap, "GP_INV");
	for(unsigned int i=0; i<device->GetInverterCount(); i++)
		MakeNode(inv_label, device->GetInverter(i), dgraph);
	
	//Make device nodes for the shift registers
	uint32_t shreg_label  = AllocateLabel(ngraph, dgraph, lmap, "GP_SHREG");
	for(unsigned int i=0; i<device->GetShiftRegisterCount(); i++)
		MakeNode(shreg_label, device->GetShiftRegister(i), dgraph);
	
	//Make device nodes for the voltage references
	uint32_t vref_label  = AllocateLabel(ngraph, dgraph, lmap, "GP_VREF");
	for(unsigned int i=0; i<device->GetVrefCount(); i++)
		MakeNode(vref_label, device->GetVref(i), dgraph);
	
	//Make device nodes for the comparators
	uint32_t acmp_label  = AllocateLabel(ngraph, dgraph, lmap, "GP_ACMP");
	for(unsigned int i=0; i<device->GetAcmpCount(); i++)
		MakeNode(acmp_label, device->GetAcmp(i), dgraph);
		
	//Make device nodes for the DACs
	uint32_t dac_label  = AllocateLabel(ngraph, dgraph, lmap, "GP_DAC");
	for(unsigned int i=0; i<device->GetDACCount(); i++)
		MakeNode(dac_label, device->GetDAC(i), dgraph);
	
	//Make device nodes for each type of flipflop
	uint32_t dff_label = AllocateLabel(ngraph, dgraph, lmap, "GP_DFF");
	uint32_t dffsr_label = AllocateLabel(ngraph, dgraph, lmap, "GP_DFFSR");
	for(unsigned int i=0; i<device->GetTotalFFCount(); i++)
	{
		Greenpak4Flipflop* flop = device->GetFlipflopByIndex(i);
		if(flop->HasSetReset())
		{
			auto node = MakeNode(dffsr_label, flop, dgraph);

			//It's legal to map a DFF to a DFFSR site, so add that as an alternate
			node->AddAlternateLabel(dff_label);
		}
		else
			MakeNode(dff_label, flop, dgraph);
	}
	
	//Make device nodes for all of the single-instance cells
	MakeSingleNode("GP_ABUF",		device->GetAbuf(), ngraph, dgraph, lmap);
	MakeSingleNode("GP_BANDGAP",	device->GetBandgap(), ngraph, dgraph, lmap);
	MakeSingleNode("GP_LFOSC",		device->GetLFOscillator(), ngraph, dgraph, lmap);
	MakeSingleNode("GP_PGA",		device->GetPGA(), ngraph, dgraph, lmap);
	MakeSingleNode("GP_POR",		device->GetPowerOnReset(), ngraph, dgraph, lmap);
	MakeSingleNode("GP_RCOSC",		device->GetRCOscillator(), ngraph, dgraph, lmap);
	MakeSingleNode("GP_RINGOSC",	device->GetRingOscillator(), ngraph, dgraph, lmap);
	MakeSingleNode("GP_SYSRESET",	device->GetSystemReset(), ngraph, dgraph, lmap);
	
	//Make device nodes for the power rails
	MakeSingleNode("GP_VDD",	device->GetPowerRail(true), ngraph, dgraph, lmap);
	MakeSingleNode("GP_VSS",	device->GetPowerRail(false), ngraph, dgraph, lmap);
	
	//Make device nodes for the counters
	uint32_t count8_label = AllocateLabel(ngraph, dgraph, lmap, "GP_COUNT8");
	uint32_t count8_adv_label = AllocateLabel(ngraph, dgraph, lmap, "GP_COUNT8_ADV");
	uint32_t count14_label = AllocateLabel(ngraph, dgraph, lmap, "GP_COUNT14");
	uint32_t count14_adv_label = AllocateLabel(ngraph, dgraph, lmap, "GP_COUNT14_ADV");
	for(unsigned int i=0; i<device->GetCounterCount(); i++)
	{
		auto counter = device->GetCounter(i);
		
		//Decide on primary label
		if(counter->GetDepth() == 14)
		{
			if(counter->HasFSM()) {
				auto node = MakeNode(count14_adv_label, counter, dgraph);
							
				//It's legal to map a COUNT8 or a COUNT14 to a COUNT14_ADV site, so add that as an alternate.
				//It's not legal to map a COUNT8_ADV to a COUNT14_ADV site because they have different behavior
				//when counting up.
				node->AddAlternateLabel(count8_label);
				node->AddAlternateLabel(count14_label);
			} else {
				auto node = MakeNode(count14_label, counter, dgraph);
							
				//It's legal to map a COUNT8 to a COUNT14 site, so add that as an alternate
				node->AddAlternateLabel(count8_label);
			}
		}
		else {
			if(counter->HasFSM()) {
				auto node = MakeNode(count8_adv_label, counter, dgraph);

				//It's legal to map a COUNT8 to a COUNT8_ADV site, so add that as an alternate.
				node->AddAlternateLabel(count8_label);
			} else {
				MakeNode(count8_label, counter, dgraph);
			}
		}
	}
	
	//TODO: make nodes for all of the other hard IP
	
	//Build inverse label map
	map<string, uint32_t> ilmap;
	for(auto it : lmap)
		ilmap[it.second] = it.first;
	
	//Add aliases for different primitive names that map to the same node type
	ilmap["GP_DFFR"] = dffsr_label;
	ilmap["GP_DFFS"] = dffsr_label;
	ilmap["GP_DFFSR"] = dffsr_label;
	
	//Make netlist nodes for cells
	for(auto it = module->cell_begin(); it != module->cell_end(); it ++)
	{
		//Figure out the type of node
		Greenpak4NetlistCell* cell = it->second;
		uint32_t label = 0;
		if(ilmap.find(cell->m_type) != ilmap.end())
			label = ilmap[cell->m_type];
		else
		{
			LogError(
				"Cell \"%s\" is of type \"%s\" which is not a valid GreenPak4 primitive\n",
				cell->m_name.c_str(), cell->m_type.c_str());
			exit(-1);			
		}
		
		//Create a node for the cell
		PARGraphNode* nnode = new PARGraphNode(label, cell);
		cell->m_parnode = nnode;
		ngraph->AddNode(nnode);
	}
	
	//Create edges in the netlist.
	//This requires breaking point-to-multipoint nets into multiple point-to-point links.
	MakeNetlistEdges(netlist);
		
	//Create edges in the device. This is static for all designs (TODO cache somehow?)
	MakeDeviceEdges(device);
}

/**
	@brief Make all of the edges in the netlist
 */
void MakeNetlistEdges(Greenpak4Netlist* netlist)
{
	LogDebug("Creating PAR netlist...\n");
	
	for(auto it = netlist->nodebegin(); it != netlist->nodeend(); it ++)
	{
		Greenpak4NetlistNode* node = *it;
			
		LogDebug("    Node %s is sourced by:\n", node->m_name.c_str());
		
		PARGraphNode* source = NULL;
		string sourceport = "";
		
		//Nets sourced by port are special - no edges
		bool sourced_by_port = false;
		for(auto p : node->m_ports)
		{
			if(p->m_direction != Greenpak4NetlistPort::DIR_OUTPUT)
			{
				//Greenpak4NetlistNet* net = netlist->GetTopModule()->GetNet(p->m_name);
				//LogVerbose("        port %s (loc %s)\n", p->m_name.c_str(), net->m_attributes["LOC"].c_str());
				sourced_by_port = true;
			}
		}
		
		//See if it was sourced by a node
		for(auto c : node->m_nodeports)
		{
			Greenpak4NetlistModule* module = netlist->GetModule(c.m_cell->m_type);
			Greenpak4NetlistPort* port = module->GetPort(c.m_portname);
			
			if(port->m_direction == Greenpak4NetlistPort::DIR_INPUT)
				continue;
			
			source = c.m_cell->m_parnode;
			sourceport = c.m_portname;
			LogDebug("        cell %s port %s\n", c.m_cell->m_name.c_str(), c.m_portname.c_str());
		}
		
		if((source == NULL) && !sourced_by_port)
			LogDebug("        [NULL]\n");
		LogDebug("        and drives\n");
		
		//If node is sourced by a port, special processing needed.
		//We can only drive IBUF/IOBUF cells
		bool has_loads = false;
		if(sourced_by_port)
		{
			if(node->m_ports.size() != 1)
			{
				LogError(
					"Net \"%s\" is connected directly to multiple top-level ports (need an IOB)\n",
					node->m_name.c_str());
				exit(-1);
			}
			
			for(auto c : node->m_nodeports)
			{
				has_loads = true;
				LogDebug("        cell %s port %s\n", c.m_cell->m_name.c_str(), c.m_portname.c_str());
				
				//Verify the type is IBUF/IOBUF
				if( (c.m_cell->m_type == "GP_IBUF") || (c.m_cell->m_type == "GP_IOBUF") )
					continue;
				
				LogError(
					"Net \"%s\" directly drives cell %s port %s (type %s, should be IOB)\n",
					node->m_name.c_str(),
					c.m_cell->m_name.c_str(),
					c.m_portname.c_str(),
					c.m_cell->m_type.c_str()
					);
				exit(-1);
			}
		}
		
		//Create edges from this source node to all sink nodes
		else
		{
			/*
			//TODO: dead code, can we delete?
			for(auto p : node->m_ports)
			{
				if(p->m_parnode != source)
				{
					source->AddEdge(sourceport, p->m_parnode);
					//Greenpak4NetlistNet* net = netlist->GetTopModule()->GetNet(p->m_name);
					//LogVerbose("        port %s (loc %s)\n", p->m_name.c_str(), net->m_attributes["LOC"].c_str());
				}
			}
			*/
			for(auto c : node->m_nodeports)
			{
				Greenpak4NetlistModule* module = netlist->GetModule(c.m_cell->m_type);
				Greenpak4NetlistPort* port = module->GetPort(c.m_portname);
				
				if(port->m_direction == Greenpak4NetlistPort::DIR_OUTPUT)
					continue;

				//Name the net
				string nname = c.m_portname;
				if(c.m_vector)
				{
					char tmp[256];
					snprintf(tmp, sizeof(tmp), "%s[%u]", c.m_portname.c_str(), c.m_nbit);
					nname = tmp;
				}
				
				//Use the new name
				has_loads = true;
				LogDebug("        cell %s port %s\n", c.m_cell->m_name.c_str(), nname.c_str());
				if(source)
					source->AddEdge(sourceport, c.m_cell->m_parnode, nname);
			}
		}
		
		//DRC fail if undriven net.
		//BUGFIX: undriven nets are legal if they also have no loads.
		//This is possible if, for example, some bits of a vector net were absorbed into hard IP.
		if( (source == NULL) && !sourced_by_port && has_loads)
		{
			LogError(
				"Net \"%s\" has loads, but no driver\n",
				node->m_name.c_str());
			exit(-1);	
		}
		else if(!has_loads)
			LogDebug("        [NULL]\n");
	}
}

/**
	@brief Make a PAR graph node and type for for a given bitstream entity, given that the device only has one of them
 */
void MakeSingleNode(
	string type,
	Greenpak4BitstreamEntity* entity,
	PARGraph* ngraph,
	PARGraph* dgraph,
	labelmap& lmap)
{
	uint32_t label = AllocateLabel(ngraph, dgraph, lmap, type);
	MakeNode(label, entity, dgraph);
}

/**
	@brief Make a PAR graph node and type for for a given bitstream entity
 */
PARGraphNode* MakeNode(
	uint32_t label,
	Greenpak4BitstreamEntity* entity,
	PARGraph* dgraph)
{
	PARGraphNode* node = new PARGraphNode(label, entity);
	entity->SetPARNode(node);
	dgraph->AddNode(node);
	return node;
}

/**
	@brief Make all of the edges for the device graph (list of all possible connections)
 */
void MakeDeviceEdges(Greenpak4Device* device)
{
	//Get all of the nodes in the device
	vector<PARGraphNode*> device_nodes;
	for(unsigned int i=0; i<device->GetEntityCount(); i++)
	{
		PARGraphNode* pnode = device->GetEntity(i)->GetPARNode();
		if(pnode)
			device_nodes.push_back(pnode);
	}
	
	//Add the O(n^2) edges between the main fabric nodes
	for(auto x : device_nodes)
	{
		auto oports = static_cast<Greenpak4BitstreamEntity*>(x->GetData())->GetOutputPorts();
		for(auto srcport : oports)
		{		
			for(auto y : device_nodes)
			{
				//Do not add edges to ourself (TODO: allow outputs of cell to feed its inputs?)
				if(x == y)
					continue;
					
				//Add paths to each cell input
				auto iports = static_cast<Greenpak4BitstreamEntity*>(y->GetData())->GetInputPorts();
				for(auto ip : iports)
					x->AddEdge(srcport, y, ip);
			}
		}
	}
	
	//Add dedicated routing between hard IP
	if(device->GetPart() == Greenpak4Device::GREENPAK4_SLG46620)
	{
		auto lfosc = device->GetLFOscillator()->GetPARNode();
		auto rosc = device->GetRingOscillator()->GetPARNode();
		auto rcosc = device->GetRCOscillator()->GetPARNode();
		
		//TODO: Disable clock outputs to dedicated routing in matrix 1 if SPI slave is enabled?
		
		////////////////////////////////////////////////////////////////////////////////////////////////////////////////
		// Cache some commonly used stuff
		
		auto pin2 = device->GetIOB(2)->GetPARNode();
		auto pin3 = device->GetIOB(3)->GetPARNode();
		auto pin4 = device->GetIOB(4)->GetPARNode();
		auto pin6 = device->GetIOB(6)->GetPARNode();
		auto pin7 = device->GetIOB(7)->GetPARNode();
		auto pin8 = device->GetIOB(8)->GetPARNode();
		auto pin9 = device->GetIOB(9)->GetPARNode();
		auto pin12 = device->GetIOB(12)->GetPARNode();
		auto pin13 = device->GetIOB(13)->GetPARNode();
		auto pin15 = device->GetIOB(15)->GetPARNode();
		auto pin16 = device->GetIOB(16)->GetPARNode();
		auto pin18 = device->GetIOB(18)->GetPARNode();
		auto pin19 = device->GetIOB(19)->GetPARNode();
		
		auto vdd = device->GetPowerRail(true)->GetPARNode();
		auto gnd = device->GetPowerRail(false)->GetPARNode();
		
		////////////////////////////////////////////////////////////////////////////////////////////////////////////////
		// CLOCK INPUTS TO COUNTERS
		
		PARGraphNode* cnodes[] =
		{
			device->GetCounter(0)->GetPARNode(),
			device->GetCounter(1)->GetPARNode(),
			device->GetCounter(2)->GetPARNode(),
			device->GetCounter(3)->GetPARNode(),
			device->GetCounter(4)->GetPARNode(),
			device->GetCounter(5)->GetPARNode(),
			device->GetCounter(6)->GetPARNode(),
			device->GetCounter(7)->GetPARNode(),
			device->GetCounter(8)->GetPARNode(),
			device->GetCounter(9)->GetPARNode()
		};
		
		//TODO: other clock sources
		lfosc->AddEdge("CLKOUT", cnodes[0], "CLK");
		rosc->AddEdge("CLKOUT_PREDIV", cnodes[0], "CLK");
		rcosc->AddEdge("CLKOUT_PREDIV", cnodes[0], "CLK");
		
		//TODO: other clock sources
		lfosc->AddEdge("CLKOUT", cnodes[1], "CLK");
		rosc->AddEdge("CLKOUT_PREDIV", cnodes[1], "CLK");
		rcosc->AddEdge("CLKOUT_PREDIV", cnodes[1], "CLK");
		
		//TODO: other clock sources
		lfosc->AddEdge("CLKOUT", cnodes[2], "CLK");
		rosc->AddEdge("CLKOUT_PREDIV", cnodes[2], "CLK");
		rcosc->AddEdge("CLKOUT_PREDIV", cnodes[2], "CLK");
		
		//TODO: other clock sources
		lfosc->AddEdge("CLKOUT", cnodes[3], "CLK");
		rosc->AddEdge("CLKOUT_PREDIV", cnodes[3], "CLK");
		rcosc->AddEdge("CLKOUT_PREDIV", cnodes[3], "CLK");
		
		//TODO: other clock sources
		lfosc->AddEdge("CLKOUT", cnodes[4], "CLK");
		rosc->AddEdge("CLKOUT_PREDIV", cnodes[4], "CLK");
		rcosc->AddEdge("CLKOUT_PREDIV", cnodes[4], "CLK");
		
		//TODO: other clock sources
		lfosc->AddEdge("CLKOUT", cnodes[5], "CLK");
		rosc->AddEdge("CLKOUT_PREDIV", cnodes[5], "CLK");
		rcosc->AddEdge("CLKOUT_PREDIV", cnodes[5], "CLK");
		
		//TODO: other clock sources
		lfosc->AddEdge("CLKOUT", cnodes[6], "CLK");
		rosc->AddEdge("CLKOUT_PREDIV", cnodes[6], "CLK");
		rcosc->AddEdge("CLKOUT_PREDIV", cnodes[6], "CLK");
		
		//TODO: other clock sources
		lfosc->AddEdge("CLKOUT", cnodes[7], "CLK");
		rosc->AddEdge("CLKOUT_PREDIV", cnodes[7], "CLK");
		rcosc->AddEdge("CLKOUT_PREDIV", cnodes[7], "CLK");
		
		//TODO: other clock sources
		lfosc->AddEdge("CLKOUT", cnodes[8], "CLK");
		rosc->AddEdge("CLKOUT_PREDIV", cnodes[8], "CLK");
		rcosc->AddEdge("CLKOUT_PREDIV", cnodes[8], "CLK");
		
		//TODO: other clock sources
		lfosc->AddEdge("CLKOUT", cnodes[9], "CLK");
		rosc->AddEdge("CLKOUT_PREDIV", cnodes[9], "CLK");
		rcosc->AddEdge("CLKOUT_PREDIV", cnodes[9], "CLK");
		
		////////////////////////////////////////////////////////////////////////////////////////////////////////////////
		// SYSTEM RESET
		
		//Can drive reset with ground or pin 2 only
		auto sysrst = device->GetSystemReset()->GetPARNode();		
		pin2->AddEdge("OUT", sysrst, "RST");
		gnd->AddEdge("OUT", sysrst, "RST");
		
		////////////////////////////////////////////////////////////////////////////////////////////////////////////////
		// REFERENCE OUT
		
		PARGraphNode* vrefs[] =
		{
			device->GetVref(0)->GetPARNode(),
			device->GetVref(1)->GetPARNode(),
			device->GetVref(2)->GetPARNode(),
			device->GetVref(3)->GetPARNode(),
			device->GetVref(4)->GetPARNode(),
			device->GetVref(5)->GetPARNode()
		};
		
		//VREF0/1 can drive pin 19
		vrefs[0]->AddEdge("VOUT", pin19, "IN");
		vrefs[1]->AddEdge("VOUT", pin19, "IN");
		
		//VREF2/3 can drive pin 18
		vrefs[2]->AddEdge("VOUT", pin18, "IN");
		vrefs[3]->AddEdge("VOUT", pin18, "IN");
		
		////////////////////////////////////////////////////////////////////////////////////////////////////////////////
		// REFERENCE TO COMPARATORS
		
		PARGraphNode* acmps[] =
		{
			device->GetAcmp(0)->GetPARNode(),
			device->GetAcmp(1)->GetPARNode(),
			device->GetAcmp(2)->GetPARNode(),
			device->GetAcmp(3)->GetPARNode(),
			device->GetAcmp(4)->GetPARNode(),
			device->GetAcmp(5)->GetPARNode()
		};
		
		//Any vref can drive any comparator, we hide the complexity of the actual routing structure
		//TODO: add a 6th vref for the DACs?
		for(auto acmp : acmps)
		{
			for(auto vref : vrefs)
				vref->AddEdge("VOUT", acmp, "VREF");
		}
		
		////////////////////////////////////////////////////////////////////////////////////////////////////////////////
		// INPUTS TO COMPARATORS
		
		auto pga = device->GetPGA()->GetPARNode();
		auto abuf = device->GetAbuf()->GetPARNode();
		
		//Input to buffer
		pin6->AddEdge("OUT", abuf, "IN");
		
		//Dedicated inputs for ACMP0 (none)
		
		//Dedicated inputs for acmps[1]
		pin12->AddEdge("OUT", acmps[1], "VIN");
		pga->AddEdge("VOUT", acmps[1], "VIN");
		
		//Dedicated inputs for acmps[2]
		pin13->AddEdge("OUT", acmps[2], "VIN");
		
		//Dedicated inputs for acmps[3]
		pin15->AddEdge("OUT", acmps[3], "VIN");
		pin13->AddEdge("OUT", acmps[3], "VIN");
		
		//Dedicated inputs for acmps[4]
		pin3->AddEdge("OUT", acmps[4], "VIN");
		pin15->AddEdge("OUT", acmps[4], "VIN");
		
		//Dedicated inputs for acmps[5]
		pin4->AddEdge("OUT", acmps[5], "VIN");
		
		//acmps[0] input before gain stage is fed to everything but acmps[5]
		pin6->AddEdge("OUT", acmps[0], "VIN");
		vdd->AddEdge("OUT", acmps[0], "VIN");
		abuf->AddEdge("OUT", acmps[0], "VIN");
		
		pin6->AddEdge("OUT", acmps[1], "VIN");
		vdd->AddEdge("OUT", acmps[1], "VIN");
		abuf->AddEdge("OUT", acmps[1], "VIN");
		
		pin6->AddEdge("OUT", acmps[2], "VIN");
		vdd->AddEdge("OUT", acmps[2], "VIN");
		abuf->AddEdge("OUT", acmps[2], "VIN");
		
		pin6->AddEdge("OUT", acmps[3], "VIN");
		vdd->AddEdge("OUT", acmps[3], "VIN");
		abuf->AddEdge("OUT", acmps[3], "VIN");
		
		pin6->AddEdge("OUT", acmps[4], "VIN");
		vdd->AddEdge("OUT", acmps[4], "VIN");
		abuf->AddEdge("OUT", acmps[4], "VIN");
		
		////////////////////////////////////////////////////////////////////////////////////////////////////////////////
		// INPUTS TO PGA
		
		vdd->AddEdge("OUT", pga, "VIN_P");
		pin8->AddEdge("OUT", pga, "VIN_P");
		
		pin9->AddEdge("OUT", pga, "VIN_N");
		gnd->AddEdge("OUT", pga, "VIN_N");
		//TODO: DAC output
		
		pin16->AddEdge("OUT", pga, "VIN_SEL");
		vdd->AddEdge("OUT", pga, "VIN_SEL");
		
		//TODO: Output to ADC
		
		////////////////////////////////////////////////////////////////////////////////////////////////////////////////
		// PGA to IOB
		
		pga->AddEdge("VOUT", pin7, "IN");
		
		////////////////////////////////////////////////////////////////////////////////////////////////////////////////
		// INPUTS TO DAC
		
		//Static 1/0 for register configuration
		for(size_t i=0; i<device->GetDACCount(); i++)
		{
			auto dac = device->GetDAC(i)->GetPARNode();
			
			vdd->AddEdge("OUT", dac, "DIN[0]");
			vdd->AddEdge("OUT", dac, "DIN[1]");
			vdd->AddEdge("OUT", dac, "DIN[2]");
			vdd->AddEdge("OUT", dac, "DIN[3]");
			vdd->AddEdge("OUT", dac, "DIN[4]");
			vdd->AddEdge("OUT", dac, "DIN[5]");
			vdd->AddEdge("OUT", dac, "DIN[6]");
			vdd->AddEdge("OUT", dac, "DIN[7]");
			
			gnd->AddEdge("OUT", dac, "DIN[0]");
			gnd->AddEdge("OUT", dac, "DIN[1]");
			gnd->AddEdge("OUT", dac, "DIN[2]");
			gnd->AddEdge("OUT", dac, "DIN[3]");
			gnd->AddEdge("OUT", dac, "DIN[4]");
			gnd->AddEdge("OUT", dac, "DIN[5]");
			gnd->AddEdge("OUT", dac, "DIN[6]");
			gnd->AddEdge("OUT", dac, "DIN[7]");
		}
	}
}
