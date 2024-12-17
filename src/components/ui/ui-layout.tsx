'use client'

import Link from 'next/link'
import {usePathname} from 'next/navigation'
import * as React from 'react'
import {ReactNode, Suspense, useEffect, useRef} from 'react'

import { ValidatorTable } from '../validator-table/validator-table'

export function UiLayout({ children }: { children: ReactNode }) {
  const pathname = usePathname()

  return (
    <div className="h-full flex flex-col">
      <div className="navbar bg-base-300 text-neutral-content flex-col md:flex-row space-y-2 md:space-y-0">
        <div className="flex-1">
          <Link className="btn btn-ghost normal-case text-xl" href="/">
            {/* <img className="h-4 md:h-6" alt="Logo" src="/logo.png" /> */}
            <div className="px-6 text-2xl text-white">Validator List</div>
          </Link>
        </div>
      </div>

      <div className="flex-grow w-full px-4">
        <ValidatorTable />
      </div>

      <footer className="footer footer-center p-4 bg-base-300 text-base-content">
        <aside>
        </aside>
      </footer>
    </div>
  )
}