import '@/app/ui/global.css'
import { Header } from './ui/Header'
import { Footer } from './ui/Footer'
import { Press_Start_2P } from '@next/font/google'

const pressStart = Press_Start_2P({
  weight: '400',
  subsets: ['cyrillic', 'latin'],
  variable: '--font-press-start-2p',
})

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en" className={`${pressStart.variable}`}>
      <body className="relative flex flex-col justify-between">
        <Header />
        {children}
        <Footer />
      </body>
    </html>
  )
}
