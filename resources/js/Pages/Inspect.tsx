import { RunTable } from '@/Components/RunTable'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/Components/ui/card'
import InspectLayout from '@/Layouts/InspectLayout'

function Inspect() {
    return (
        <div className="pt-12 mx-auto px-4 w-full lg:w-2/3">
            <Card>
                <CardHeader>
                    <CardTitle>Recent agent runs</CardTitle>
                    <CardDescription>Click any row to view details</CardDescription>
                </CardHeader>
                <CardContent>
                    <RunTable />
                </CardContent>
            </Card>
        </div>
    )
}

Inspect.layout = (page) => <InspectLayout children={page} title="Inspect" />

export default Inspect
