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

#ifndef Greenpak4IOBTypeB_h
#define Greenpak4IOBTypeB_h

/**
	@brief A single IOB, bitfile format type B. No output enable.
 */
class Greenpak4IOBTypeB : public Greenpak4IOB
{
public:

	//Construction / destruction
	Greenpak4IOBTypeB(
		Greenpak4Device* device,
		unsigned int pin_num,
		unsigned int matrix,
		unsigned int ibase,
		unsigned int oword,
		unsigned int cbase,
		unsigned int flags = IOB_FLAG_NORMAL);
	virtual ~Greenpak4IOBTypeB();

	//Serialization
	virtual bool Load(bool* bitstream);
	virtual bool Save(bool* bitstream);

	virtual std::string GetDescription();
};

#endif

