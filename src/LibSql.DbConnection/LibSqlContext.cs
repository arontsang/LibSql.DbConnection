using LibSql.DbConnection.Bindings;

namespace LibSql.DbConnection;

public class LibSqlContext : IDisposable
{
    private readonly LibTursoRuntime _context = LibTursoRuntime.New();
    
    public void Dispose() => _context.Dispose();
}