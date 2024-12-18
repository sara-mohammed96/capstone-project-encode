'use client'

import * as React from 'react'
import { Validator } from '../types';

export function ValidatorTable({ validators }: { validators: Validator[] }) {
  console.log(validators);
  return (
    <div className="relative overflow-x-auto shadow-2xl sm:rounded-lg m-8">
      <table className="w-full text-sm text-left rtl:text-right text-gray-500 dark:text-gray-400 shadow-xl">
        {/* Table Header */}
        <thead className="text-s uppercase bg-gray-50 dark:bg-gray-100 dark:text-gray-100">
          <tr className="bg-[rgb(73,181,193)]">
            <th scope="col" className="px-6 py-3 text-left">ID & Name</th>
            <th scope="col" className="px-6 py-3 text-left">Activated stake</th>
            <th scope="col" className="px-6 py-3 text-left">Root distance</th>
            <th scope="col" className="px-6 py-3 text-left">Vote distance</th>
            <th scope="col" className="px-6 py-3 text-left">Commission</th>
          </tr>
        </thead>
        {/* Table Body */}
        <tbody>
          {validators.map((validator) => (
            <tr key={validator.id} className="bg-[rgba(44,34,62,255)] border-b dark:border-gray-700 hover:bg-[rgba(52,40,70,255)]">
              <th scope="row" className="flex items-center px-6 py-4 text-gray-900 whitespace-nowrap dark:text-white">
                    <div className="ps-3">
                        <div className="text-base font-semibold">{validator.vote_account_info.nodePubkey}</div>
                    </div>  
              </th>
              <td className="px-6 py-4">{validator.vote_account_info.activatedStake}</td>
              <td className="px-6 py-4">{validator.root_distance}</td>
              <td className="px-6 py-4">{validator.vote_distance}</td>
              <td className="px-6 py-4">{validator.vote_account_info.commission}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  )
}
