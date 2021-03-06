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

#include <log.h>
#include <xbpar.h>
#include <Greenpak4.h>

using namespace std;

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction / destruction

Greenpak4BitstreamEntity::Greenpak4BitstreamEntity(
	Greenpak4Device* device,
	unsigned int matrix,
	unsigned int ibase,
	unsigned int obase,
	unsigned int cbase
	)
	: m_device(device)
	, m_matrix(matrix)
	, m_inputBaseWord(ibase)
	, m_outputBaseWord(obase)
	, m_configBase(cbase)
	, m_parnode(NULL)
	, m_dual(NULL)
	, m_dualMaster(true)
{

}

Greenpak4BitstreamEntity::~Greenpak4BitstreamEntity()
{
	//Delete our dual if we're the master
	if(m_dual && m_dualMaster)
	{
		delete m_dual;
		m_dual = NULL;
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Connectivity tracing helpers

/**
	@brief Check if we have any loads on a particular port
 */
bool Greenpak4BitstreamEntity::HasLoadsOnPort(string port)
{
	//Get our cell, or bail if we're unassigned
	auto ncell = dynamic_cast<Greenpak4NetlistCell*>(GetNetlistEntity());
	if(ncell == NULL)
		return false;

	//If nothing on the port, stop
	if(ncell->m_connections.find(port) == ncell->m_connections.end())
		return false;

	//Check if any connections other than ourself
	auto vec = ncell->m_connections[port];
	for(auto node : vec)
	{
		for(auto point : node->m_nodeports)
		{
			if(point.m_cell != ncell)
				return true;
		}

		if(!node->m_ports.empty())
			return true;
	}

	return false;
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Net numbering helpers

Greenpak4EntityOutput Greenpak4BitstreamEntity::GetOutput(std::string port)
{
	return Greenpak4EntityOutput(this, port, m_matrix);
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Commit helpers

Greenpak4NetlistEntity* Greenpak4BitstreamEntity::GetNetlistEntity()
{
	PARGraphNode* mate = m_parnode->GetMate();
	if(mate == NULL)
		return NULL;
	return static_cast<Greenpak4NetlistEntity*>(mate->GetData());
}

/**
	@brief Returns true if the given named port is general fabric routing
 */
bool Greenpak4BitstreamEntity::IsGeneralFabricInput(string port) const
{
	auto ports = GetInputPorts();
	for(auto p : ports)
	{
		if(p == port)
			return true;
	}
	return false;
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Debug log helpers

string Greenpak4BitstreamEntity::GetOutputName()
{
	auto mate = m_parnode->GetMate();
	if(mate == NULL)
		return "";
	auto entity = static_cast<Greenpak4NetlistEntity*>(mate->GetData());

	//If it's an IOB, return the IOB name
	if(dynamic_cast<Greenpak4NetlistPort*>(entity))
		return entity->m_name;

	//Nope, it's a cell
	auto cell = dynamic_cast<Greenpak4NetlistCell*>(entity);
	if(!cell)
		return "error";

	//Look up our first output port... HACK!
	//TODO: Fix this
	string portname = GetOutputPorts()[0];

	//Find the net we connect to
	if(cell->m_connections.find(portname) == cell->m_connections.end())
		return "error";
	return cell->m_connections[portname][0]->m_name;	//FIXME:VECTOR
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Load/save helpers

Greenpak4BitstreamEntity* Greenpak4BitstreamEntity::GetRealEntity()
{
	if(m_dual && !m_dualMaster)
		return m_dual;

	return this;
}

bool Greenpak4BitstreamEntity::WriteMatrixSelector(
	bool* bitstream,
	unsigned int wordpos,
	Greenpak4EntityOutput signal,
	bool cross_matrix)
{
	//Can't hook up non-routable signals
	if(signal.GetNetNumber() > 255)
	{
		LogError("Tried to write signal from invalid net %x\n", signal.GetNetNumber());
		return false;
	}

	//SANITY CHECK - must be attached to the same matrix
	//cross connections use opposite, though
	if(cross_matrix)
	{
		//Do not do check if the signal is a power rail (this is the case for unused cross connections)
		if(signal.IsPowerRail())
		{}

		//No other signal, dual or not, should do this
		else if(m_matrix == signal.GetMatrix())
		{
			LogError("Tried to write signal from same matrix through a cross connection\n");
			return false;
		}
	}
	else if(m_matrix != signal.GetMatrix())
	{
		//If we have a dual, use that
		if(signal.HasDual())
			signal = signal.GetDual();

		//otherwise something is fishy
		else
		{
			LogError("Tried to write signal from opposite matrix without using a cross connection\n");
			return false;
		}
	}

	//Good to go, write it
	unsigned int sel = signal.GetNetNumber();

	//Calculate right matrix for cross connections etc
	unsigned int matrix = m_matrix;
	if(cross_matrix)
		matrix = 1 - matrix;

	unsigned int nbits = m_device->GetMatrixBits();
	unsigned int startbit = m_device->GetMatrixBase(matrix) + wordpos * nbits;

	//Need to flip bit ordering since lowest array index is the MSB
	for(unsigned int i=0; i<nbits; i++)
	{
		if( (sel >> i) & 1 )
			bitstream[startbit + i] = true;
		else
			bitstream[startbit + i] = false;
	}

	return true;
}
