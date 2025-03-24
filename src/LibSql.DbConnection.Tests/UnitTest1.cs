namespace LibSql.DbConnection.Tests;

public class Tests
{

    [Test]
    public void Test1()
    {
        var ret = Foo.DoFoo(10, 20);
        Assert.AreEqual(30, ret);
    }
}