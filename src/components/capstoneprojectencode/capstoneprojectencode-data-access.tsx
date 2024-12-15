'use client'

import { getCapstoneprojectencodeProgram, getCapstoneprojectencodeProgramId } from '@project/anchor'
import { useConnection } from '@solana/wallet-adapter-react'
import { Cluster, Keypair, PublicKey } from '@solana/web3.js'
import { useMutation, useQuery } from '@tanstack/react-query'
import { useMemo } from 'react'
import toast from 'react-hot-toast'
import { useCluster } from '../cluster/cluster-data-access'
import { useAnchorProvider } from '../solana/solana-provider'
import { useTransactionToast } from '../ui/ui-layout'

export function useCapstoneprojectencodeProgram() {
  const { connection } = useConnection()
  const { cluster } = useCluster()
  const transactionToast = useTransactionToast()
  const provider = useAnchorProvider()
  const programId = useMemo(() => getCapstoneprojectencodeProgramId(cluster.network as Cluster), [cluster])
  const program = useMemo(() => getCapstoneprojectencodeProgram(provider, programId), [provider, programId])

  const accounts = useQuery({
    queryKey: ['capstoneprojectencode', 'all', { cluster }],
    queryFn: () => program.account.capstoneprojectencode.all(),
  })

  const getProgramAccount = useQuery({
    queryKey: ['get-program-account', { cluster }],
    queryFn: () => connection.getParsedAccountInfo(programId),
  })

  const initialize = useMutation({
    mutationKey: ['capstoneprojectencode', 'initialize', { cluster }],
    mutationFn: (keypair: Keypair) =>
      program.methods.initialize().accounts({ capstoneprojectencode: keypair.publicKey }).signers([keypair]).rpc(),
    onSuccess: (signature) => {
      transactionToast(signature)
      return accounts.refetch()
    },
    onError: () => toast.error('Failed to initialize account'),
  })

  return {
    program,
    programId,
    accounts,
    getProgramAccount,
    initialize,
  }
}

export function useCapstoneprojectencodeProgramAccount({ account }: { account: PublicKey }) {
  const { cluster } = useCluster()
  const transactionToast = useTransactionToast()
  const { program, accounts } = useCapstoneprojectencodeProgram()

  const accountQuery = useQuery({
    queryKey: ['capstoneprojectencode', 'fetch', { cluster, account }],
    queryFn: () => program.account.capstoneprojectencode.fetch(account),
  })

  const closeMutation = useMutation({
    mutationKey: ['capstoneprojectencode', 'close', { cluster, account }],
    mutationFn: () => program.methods.close().accounts({ capstoneprojectencode: account }).rpc(),
    onSuccess: (tx) => {
      transactionToast(tx)
      return accounts.refetch()
    },
  })

  const decrementMutation = useMutation({
    mutationKey: ['capstoneprojectencode', 'decrement', { cluster, account }],
    mutationFn: () => program.methods.decrement().accounts({ capstoneprojectencode: account }).rpc(),
    onSuccess: (tx) => {
      transactionToast(tx)
      return accountQuery.refetch()
    },
  })

  const incrementMutation = useMutation({
    mutationKey: ['capstoneprojectencode', 'increment', { cluster, account }],
    mutationFn: () => program.methods.increment().accounts({ capstoneprojectencode: account }).rpc(),
    onSuccess: (tx) => {
      transactionToast(tx)
      return accountQuery.refetch()
    },
  })

  const setMutation = useMutation({
    mutationKey: ['capstoneprojectencode', 'set', { cluster, account }],
    mutationFn: (value: number) => program.methods.set(value).accounts({ capstoneprojectencode: account }).rpc(),
    onSuccess: (tx) => {
      transactionToast(tx)
      return accountQuery.refetch()
    },
  })

  return {
    accountQuery,
    closeMutation,
    decrementMutation,
    incrementMutation,
    setMutation,
  }
}
