'use client'

import Link from 'next/link'
import { usePathname } from 'next/navigation'
import * as React from 'react'
import { ReactNode, useEffect, useState } from 'react'

import { ValidatorTable } from '../validator-table/validator-table'

const api = "http://127.0.0.1:3030";

export function UiLayout({ children }: { children: ReactNode }) {
  const [validators, setValidators] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  useEffect(() => {
    const fetchData = async () => {
      try {
        const response = await fetch(`${api}/validators`);
        if (!response.ok) {
          throw new Error('Failed to fetch data');
        }
        const data = await response.json();
        console.log("data", data);
        setValidators(data);
      } catch (err) {
        setError(err.message);
        setValidators(validators);
      } finally {
        setLoading(false);
      }
    };

    fetchData();
  }, []);

  if (loading) return <div>Loading...</div>;
  if (error) return <div>Error: {error}</div>;

  return (
    <div className="h-full flex flex-col">
      <div className="navbar bg-base-300 text-neutral-content flex-col md:flex-row space-y-2 md:space-y-0">
        <div className="flex-1">
          <Link className="btn btn-ghost normal-case text-xl" href="/">
            <div className="px-6 text-2xl text-white">Validator List</div>
          </Link>
        </div>
      </div>

      <div className="flex-grow w-full px-4">
        <ValidatorTable validators={validators}/>
      </div>

      <footer className="footer footer-center p-4 bg-base-300 text-base-content">
        <aside>
        </aside>
      </footer>
    </div>
  )
}