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
#include <Greenpak4.h>

using namespace std;

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction / destruction

Greenpak4NetlistCell::~Greenpak4NetlistCell()
{
	//do not delete wires, module dtor handles that
}

string Greenpak4NetlistCell::GetLOC()
{
	string loc = m_attributes.at("LOC");

	//If we're an IOB, do vector processing
	if(IsIOB())
	{
		//Get the top-level pad signal as this is always the vector
		string port;
		if( (m_type == "GP_OBUF") || (m_type == "GP_OBUFT") )
			port = "OUT";
		else if(m_type == "GP_IBUF")
			port = "IN";
		else if(m_type == "GP_IOBUF")
			port = "IO";
		auto cn = m_connections[port];

		//Should be a scalar, get the single bit
		if(cn.empty())
			return "<invalid LOC>";
		auto nodename = cn[0]->m_name;

		//If the node name is a scalar, return the LOC as is
		size_t pos = nodename.find("[");
		if(pos == string::npos)
			return loc;

		//It's a vector... get the bit index
		string sindex = nodename.substr(pos + 1);	//skip the [ character
		int index = atoi(sindex.c_str());

		//Find our LOC
		pos = 0;
		for(int i=0; i<index; i++)
		{
			pos = loc.find(" ", pos);
			if(pos == string::npos)
				return "<invalid LOC>";

			pos++;	//skip the space
		}
		size_t nextpos = loc.find(" ", pos);
		if(nextpos == string::npos)
			nextpos = loc.length();
		nextpos -= pos;

		return loc.substr(pos, nextpos);
	}

	//Single bit signal, just return the attribute
	else
		return loc;
}
