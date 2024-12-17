'use client'

import * as React from 'react'
import {ReactNode, Suspense, useEffect, useRef} from 'react'

interface Validator {
  id: number
  name: string
  uptime: string
  fees: string
  location: string
}

const validators: Validator[] = [
  { id: 1, name: 'Validator Alpha', uptime: '99.9%', fees: '5%', location: 'USA' },
  { id: 2, name: 'Validator Beta', uptime: '98.5%', fees: '7%', location: 'Europe' },
  { id: 3, name: 'Validator Gamma', uptime: '99.7%', fees: '4.5%', location: 'Asia' },
  { id: 4, name: 'Validator Delta', uptime: '97.9%', fees: '6%', location: 'South America' },
  { id: 5, name: 'Validator Epsilon', uptime: '99.2%', fees: '5.5%', location: 'Africa' },
  { id: 3, name: 'Validator Gamma', uptime: '99.7%', fees: '4.5%', location: 'Asia' },
  { id: 4, name: 'Validator Delta', uptime: '97.9%', fees: '6%', location: 'South America' },
  { id: 5, name: 'Validator Epsilon', uptime: '99.2%', fees: '5.5%', location: 'Africa' },
  { id: 3, name: 'Validator Gamma', uptime: '99.7%', fees: '4.5%', location: 'Asia' },
  { id: 4, name: 'Validator Delta', uptime: '97.9%', fees: '6%', location: 'South America' },
  { id: 5, name: 'Validator Epsilon', uptime: '99.2%', fees: '5.5%', location: 'Africa' },
]

export function ValidatorTable() {
  return (
    <div className="relative overflow-x-auto shadow-md sm:rounded-lg m-8">
      <table className="w-full text-sm text-left rtl:text-right text-gray-500 dark:text-gray-400">
        {/* Table Header */}
        <thead className="text-s uppercase bg-gray-50 dark:bg-gray-100 dark:text-gray-100">
          <tr className="bg-[rgb(73,181,193)]">
            <th scope="col" className="px-6 py-3 text-left">ID & Name</th>
            <th scope="col" className="px-6 py-3 text-left">Uptime</th>
            <th scope="col" className="px-6 py-3 text-left">Fees</th>
            <th scope="col" className="px-6 py-3 text-left">Location</th>
          </tr>
        </thead>
        {/* Table Body */}
        <tbody>
          {validators.map((validator) => (
            <tr key={validator.id} className="bg-[rgba(44,34,62,255)] border-b dark:border-gray-700 hover:bg-[rgba(52,40,70,255)]">
              <th scope="row" className="flex items-center px-6 py-4 text-gray-900 whitespace-nowrap dark:text-white">
                    {/* <img className="w-10 h-10 rounded-full" src="/docs/images/people/profile-picture-1.jpg" alt="Jese image"></img> */}
                    <div className="ps-3">
                        <div className="text-base font-semibold">{validator.id}</div>
                        <div className="font-normal text-gray-500">{validator.name}</div>
                    </div>  
              </th>
              {/* <td className="border border-gray-300 px-4 py-2">{validator.id}</td>
              <td className="border border-gray-300 px-4 py-2">{validator.name}</td> */}
              <td className="px-6 py-4">{validator.uptime}</td>
              <td className="px-6 py-4">{validator.fees}</td>
              <td className="px-6 py-4">{validator.location}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  )
}
